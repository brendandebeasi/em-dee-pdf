#!/usr/bin/env python3
"""Render a specific page of a PDF to PNG using macOS native CoreGraphics.

Produces crisp, high-resolution screenshots with proper font smoothing.
Used for generating README theme gallery images.

Requires: pip install pyobjc-framework-Quartz (macOS only)

Usage:
    python3 scripts/render-pdf-page.py <pdf_path> <page_num> <output.png> [scale]

Examples:
    python3 scripts/render-pdf-page.py examples/slate.pdf 1 examples/screenshots/slate.png
    python3 scripts/render-pdf-page.py examples/coral.pdf 2 examples/screenshots/coral.png 4
"""
import sys

try:
    import Quartz
    import CoreFoundation
except ImportError:
    print("Error: pyobjc-framework-Quartz is required (macOS only)")
    print("Install: pip install pyobjc-framework-Quartz")
    sys.exit(1)


def render_pdf_page(pdf_path, page_num, output_path, scale=3.0):
    url = CoreFoundation.CFURLCreateFromFileSystemRepresentation(
        None, pdf_path.encode(), len(pdf_path.encode()), False
    )
    pdf = Quartz.CGPDFDocumentCreateWithURL(url)
    if pdf is None:
        print(f"Error: Could not open {pdf_path}")
        sys.exit(1)

    num_pages = Quartz.CGPDFDocumentGetNumberOfPages(pdf)
    if page_num < 1 or page_num > num_pages:
        print(f"Error: Page {page_num} out of range (1-{num_pages})")
        sys.exit(1)

    page = Quartz.CGPDFDocumentGetPage(pdf, page_num)
    page_rect = Quartz.CGPDFPageGetBoxRect(page, Quartz.kCGPDFMediaBox)

    width = int(page_rect.size.width * scale)
    height = int(page_rect.size.height * scale)

    cs = Quartz.CGColorSpaceCreateDeviceRGB()
    ctx = Quartz.CGBitmapContextCreate(
        None, width, height, 8, width * 4, cs,
        Quartz.kCGImageAlphaPremultipliedFirst | Quartz.kCGBitmapByteOrder32Little
    )

    # White background
    Quartz.CGContextSetRGBFillColor(ctx, 1.0, 1.0, 1.0, 1.0)
    Quartz.CGContextFillRect(ctx, Quartz.CGRectMake(0, 0, width, height))

    # Enable font smoothing and antialiasing
    Quartz.CGContextSetShouldAntialias(ctx, True)
    Quartz.CGContextSetAllowsAntialiasing(ctx, True)
    Quartz.CGContextSetShouldSmoothFonts(ctx, True)
    Quartz.CGContextSetAllowsFontSmoothing(ctx, True)
    Quartz.CGContextSetInterpolationQuality(ctx, Quartz.kCGInterpolationHigh)

    # Scale and render
    Quartz.CGContextScaleCTM(ctx, scale, scale)
    Quartz.CGContextDrawPDFPage(ctx, page)

    # Save as PNG
    image = Quartz.CGBitmapContextCreateImage(ctx)
    out_url = CoreFoundation.CFURLCreateFromFileSystemRepresentation(
        None, output_path.encode(), len(output_path.encode()), False
    )
    dest = Quartz.CGImageDestinationCreateWithURL(out_url, "public.png", 1, None)
    Quartz.CGImageDestinationAddImage(dest, image, None)
    Quartz.CGImageDestinationFinalize(dest)
    print(f"Rendered {pdf_path} page {page_num} -> {output_path} ({width}x{height})")


if __name__ == "__main__":
    if len(sys.argv) < 4 or len(sys.argv) > 5:
        print(f"Usage: {sys.argv[0]} <pdf_path> <page_num> <output.png> [scale]")
        sys.exit(1)
    scale = float(sys.argv[4]) if len(sys.argv) == 5 else 3.0
    render_pdf_page(sys.argv[1], int(sys.argv[2]), sys.argv[3], scale)
