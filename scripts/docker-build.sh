#!/bin/bash
# Build the md-pdf Docker image

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

docker build -t md-pdf:latest .

echo ""
echo "Built md-pdf:latest"
echo ""
echo "Usage:"
echo "  docker run --rm -v \"\$(pwd):/work\" md-pdf:latest input.md --emit-typst"
echo "  docker run --rm -v \"\$(pwd):/work\" md-pdf:latest input.md -o output.pdf"
