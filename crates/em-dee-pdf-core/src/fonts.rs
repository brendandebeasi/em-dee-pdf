//! Font resolution: detect which families a theme asks for, find what the
//! system already has, optionally download the rest from Google Fonts (with the
//! user's consent), and warn whenever a requested font can't be satisfied.
//!
//! The renderer hands every loaded font blob to Typst. Typst silently falls back
//! to whatever face it can find when a requested family is missing, which is how
//! a clean document ends up rendered in a spaced-out monospace face. This module
//! closes that gap: it makes the fallback explicit (a warning) and gives the user
//! a way to pull the intended font instead.

use std::collections::BTreeSet;
use std::io::Read;
use std::path::PathBuf;
use std::time::Duration;

use tracing::{info, warn};

pub use crate::config::DownloadPolicy;

/// CSS generic family keywords. These never name a real installed face, so we
/// skip them when deciding whether a font stack is satisfiable.
const GENERIC_FAMILIES: &[&str] = &[
    "serif",
    "sans-serif",
    "monospace",
    "cursive",
    "fantasy",
    "system-ui",
    "ui-serif",
    "ui-sans-serif",
    "ui-monospace",
];

/// User-Agent that makes the Google Fonts CSS API return plain TrueType (`.ttf`)
/// URLs instead of `woff2` (which Typst can't consume) or the extension-less
/// legacy `/l/font?kit=` endpoints that newer-browser UAs receive.
const TTF_USER_AGENT: &str = "Mozilla/4.0";

/// Collect the set of font family names (lowercased) present in the given font
/// blobs, covering both the legacy family name (id 1) and the typographic family
/// name (id 16).
pub fn available_families(blobs: &[Vec<u8>]) -> BTreeSet<String> {
    let mut families = BTreeSet::new();
    for blob in blobs {
        let faces = ttf_parser::fonts_in_collection(blob).unwrap_or(1);
        for index in 0..faces {
            let Ok(face) = ttf_parser::Face::parse(blob, index) else {
                continue;
            };
            for name in face.names() {
                if name.name_id == ttf_parser::name_id::FAMILY
                    || name.name_id == ttf_parser::name_id::TYPOGRAPHIC_FAMILY
                {
                    if let Some(value) = name.to_string() {
                        families.insert(value.trim().to_lowercase());
                    }
                }
            }
        }
    }
    families
}

/// Parse the font stacks a theme preamble requests. Recognises both
/// `font: ("A", "B")` and `font: "A"`, reading one logical line at a time (the
/// built-in themes keep each `font:` declaration on a single line).
pub fn requested_stacks(preamble: &str) -> Vec<Vec<String>> {
    let mut stacks = Vec::new();
    let mut cursor = 0;
    while let Some(found) = preamble[cursor..].find("font:") {
        let start = cursor + found + "font:".len();
        let line_end = preamble[start..]
            .find('\n')
            .map(|e| start + e)
            .unwrap_or(preamble.len());
        let families = extract_quoted(&preamble[start..line_end]);
        if !families.is_empty() {
            stacks.push(families);
        }
        cursor = line_end;
    }
    stacks
}

/// Resolve every requested font stack against what's installed, downloading
/// missing primaries when allowed. Returns any extra font blobs that were
/// fetched so the caller can hand them to the renderer alongside system fonts.
pub fn resolve(
    theme_name: &str,
    preamble: &str,
    available: &BTreeSet<String>,
    policy: DownloadPolicy,
    cache_dir: Option<PathBuf>,
) -> Vec<Vec<u8>> {
    let mut extra: Vec<Vec<u8>> = Vec::new();
    let mut newly_available: BTreeSet<String> = BTreeSet::new();
    // Families we've already acted on this run, so a font used by several stacks
    // (e.g. body + headings) is only prompted for, fetched, or warned about once.
    let mut handled: BTreeSet<String> = BTreeSet::new();

    let has = |family: &str, extra_set: &BTreeSet<String>| {
        let key = family.to_lowercase();
        available.contains(&key) || extra_set.contains(&key)
    };

    for stack in requested_stacks(preamble) {
        let concrete: Vec<&String> = stack
            .iter()
            .filter(|f| !GENERIC_FAMILIES.contains(&f.to_lowercase().as_str()))
            .collect();
        let Some(primary) = concrete.first().copied() else {
            continue;
        };

        if has(primary, &newly_available) {
            continue;
        }
        let primary_key = primary.to_lowercase();
        if !handled.insert(primary_key) {
            continue;
        }

        let fallback = concrete[1..]
            .iter()
            .find(|f| has(f, &newly_available))
            .map(|f| f.to_string());

        let consent = match policy {
            DownloadPolicy::Always => true,
            DownloadPolicy::Never => false,
            DownloadPolicy::Prompt => prompt_consent(primary),
        };

        if consent {
            match fetch_family(primary, cache_dir.as_ref()) {
                Some(blobs) => {
                    info!(
                        theme = theme_name,
                        font = primary,
                        files = blobs.len(),
                        "downloaded missing font"
                    );
                    extra.extend(blobs);
                    newly_available.insert(primary.to_lowercase());
                    continue;
                }
                None => warn!(
                    "font \"{}\" (theme \"{}\") is not installed and could not be downloaded from Google Fonts{}",
                    primary,
                    theme_name,
                    describe_fallback(&fallback)
                ),
            }
        } else {
            warn!(
                "font \"{}\" (theme \"{}\") is not installed{}",
                primary,
                theme_name,
                describe_fallback(&fallback)
            );
        }
    }

    extra
}

/// Render the trailing clause of a warning describing what will be used instead.
fn describe_fallback(fallback: &Option<String>) -> String {
    match fallback {
        Some(name) => format!("; falling back to \"{}\"", name),
        None => {
            "; no fallback in the stack is installed either, so text may render with a default face"
                .to_string()
        }
    }
}

/// Ask the user whether to download a font. Only prompts on an interactive
/// terminal; in a pipe / CI it declines and tells the user how to opt in.
fn prompt_consent(family: &str) -> bool {
    use std::io::{IsTerminal, Write};

    if !std::io::stdin().is_terminal() {
        warn!(
            "font \"{}\" is not installed; re-run with --download-fonts to fetch it from Google Fonts (no terminal to prompt on, skipping)",
            family
        );
        return false;
    }

    eprint!(
        "em-dee-pdf: font \"{}\" is not installed. Download it from Google Fonts? [y/N] ",
        family
    );
    let _ = std::io::stderr().flush();

    let mut answer = String::new();
    if std::io::stdin().read_line(&mut answer).is_err() {
        return false;
    }
    matches!(answer.trim().to_lowercase().as_str(), "y" | "yes")
}

/// Fetch a family's ttf files, preferring the on-disk cache. A previous failed
/// lookup is remembered (a `.notfound` marker) so we don't hit the network for
/// the same unavailable family on every run.
fn fetch_family(family: &str, cache_dir: Option<&PathBuf>) -> Option<Vec<Vec<u8>>> {
    let slug = slugify(family);

    if let Some(dir) = cache_dir {
        let family_dir = dir.join(&slug);
        if family_dir.join(".notfound").exists() {
            return None;
        }
        let cached = read_ttfs(&family_dir);
        if !cached.is_empty() {
            return Some(cached);
        }
    }

    let blobs = download_family(family);

    match (&blobs, cache_dir) {
        (Some(files), Some(dir)) => {
            let family_dir = dir.join(&slug);
            let _ = std::fs::create_dir_all(&family_dir);
            for (i, bytes) in files.iter().enumerate() {
                let _ = std::fs::write(family_dir.join(format!("{slug}-{i}.ttf")), bytes);
            }
        }
        (None, Some(dir)) => {
            let family_dir = dir.join(&slug);
            let _ = std::fs::create_dir_all(&family_dir);
            let _ = std::fs::write(family_dir.join(".notfound"), b"");
        }
        _ => {}
    }

    blobs
}

/// Download all ttf weights Google Fonts offers for a family.
fn download_family(family: &str) -> Option<Vec<Vec<u8>>> {
    let css_url = format!(
        "https://fonts.googleapis.com/css2?family={}:wght@400;500;600;700",
        family.replace(' ', "+")
    );
    let css = http_get_text(&css_url)?;

    let mut blobs = Vec::new();
    let mut seen = BTreeSet::new();
    for url in extract_font_urls(&css) {
        if !seen.insert(url.clone()) {
            continue;
        }
        if let Some(bytes) = http_get_bytes(&url) {
            blobs.push(bytes);
        }
    }

    if blobs.is_empty() {
        None
    } else {
        Some(blobs)
    }
}

fn http_get_text(url: &str) -> Option<String> {
    ureq::get(url)
        .set("User-Agent", TTF_USER_AGENT)
        .timeout(Duration::from_secs(15))
        .call()
        .ok()?
        .into_string()
        .ok()
}

fn http_get_bytes(url: &str) -> Option<Vec<u8>> {
    let response = ureq::get(url)
        .timeout(Duration::from_secs(30))
        .call()
        .ok()?;
    let mut buffer = Vec::new();
    response.into_reader().read_to_end(&mut buffer).ok()?;
    Some(buffer)
}

/// Pull `https://….ttf` / `.otf` URLs out of a Google Fonts CSS response.
fn extract_font_urls(css: &str) -> Vec<String> {
    let mut urls = Vec::new();
    let mut rest = css;
    while let Some(open) = rest.find("url(") {
        let after = &rest[open + 4..];
        let Some(close) = after.find(')') else {
            break;
        };
        let raw = after[..close].trim_matches(|c| c == '"' || c == '\'');
        let lower = raw.to_lowercase();
        if raw.starts_with("https://") && (lower.ends_with(".ttf") || lower.ends_with(".otf")) {
            urls.push(raw.to_string());
        }
        rest = &after[close + 1..];
    }
    urls
}

/// Collect ttf/otf blobs from a directory (non-recursive).
fn read_ttfs(dir: &PathBuf) -> Vec<Vec<u8>> {
    let mut blobs = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else {
        return blobs;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());
        if matches!(ext.as_deref(), Some("ttf") | Some("otf")) {
            if let Ok(bytes) = std::fs::read(&path) {
                blobs.push(bytes);
            }
        }
    }
    blobs
}

/// Filesystem-safe slug for a family name (used as a cache subdirectory).
fn slugify(family: &str) -> String {
    family
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

/// Extract the quoted string literals from a snippet, in order.
fn extract_quoted(segment: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = segment.char_indices().peekable();
    while let Some((_, c)) = chars.next() {
        if c == '"' {
            let mut value = String::new();
            for (_, c2) in chars.by_ref() {
                if c2 == '"' {
                    break;
                }
                value.push(c2);
            }
            if !value.trim().is_empty() {
                out.push(value.trim().to_string());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_single_and_list_font_declarations() {
        let preamble = r#"
            #set text(font: ("Inter", "Helvetica Neue", "sans-serif"))
            #show raw: set text(font: "JetBrains Mono")
        "#;
        let stacks = requested_stacks(preamble);
        assert_eq!(stacks.len(), 2);
        assert_eq!(stacks[0], vec!["Inter", "Helvetica Neue", "sans-serif"]);
        assert_eq!(stacks[1], vec!["JetBrains Mono"]);
    }

    #[test]
    fn extracts_only_ttf_and_otf_urls() {
        let css =
            "src: url(https://x/a.woff2) format('woff2'), url(https://x/b.ttf) format('truetype');";
        assert_eq!(extract_font_urls(css), vec!["https://x/b.ttf"]);
    }

    #[test]
    fn slugify_is_filesystem_safe() {
        assert_eq!(slugify("Source Sans Pro"), "source-sans-pro");
        assert_eq!(slugify("JetBrains Mono"), "jetbrains-mono");
    }
}
