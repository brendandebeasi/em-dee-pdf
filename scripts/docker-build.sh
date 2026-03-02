#!/bin/bash
# Build the em-dee-pdf Docker image

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

docker build -t em-dee-pdf:latest .

echo ""
echo "Built em-dee-pdf:latest"
echo ""
echo "Usage:"
echo "  docker run --rm -v \"\$(pwd):/work\" em-dee-pdf:latest input.md --emit-typst"
echo "  docker run --rm -v \"\$(pwd):/work\" em-dee-pdf:latest input.md -o output.pdf"
