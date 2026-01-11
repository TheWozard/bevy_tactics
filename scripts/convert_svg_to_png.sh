#!/bin/bash

# Script to convert all SVG files to PNG format
# Requires rsvg-convert (install via: brew install librsvg)
# Usage: convert_svg_to_png.sh <scale> <svg_dir> [output_dir]

# Configuration
SCALE="$1" # Scale as number of pixels per 100 units
SVG_DIR="$2"
OUTPUT_DIR="${3:-$SVG_DIR}"

# Check for required arguments
if [ -z "$SCALE" ] || [ -z "$SVG_DIR" ]; then
    echo "Usage: $0 <scale> <svg_dir> [output_dir]"
    echo "  scale: Number of pixels per 100 SVG units"
    echo "  svg_dir: Directory containing SVG files"
    echo "  output_dir: Optional output directory (defaults to svg_dir)"
    exit 1
fi

# Check if rsvg-convert is installed
if ! command -v rsvg-convert &> /dev/null; then
    echo "Error: rsvg-convert is not installed"
    echo "Install it using: brew install librsvg"
    exit 1
fi

# Check if SVG directory exists
if [ ! -d "$SVG_DIR" ]; then
    echo "Error: Directory $SVG_DIR does not exist"
    exit 1
fi

# Create output directory if it doesn't exist
mkdir -p "$OUTPUT_DIR"

# Counter for converted files
count=0
SCALE=$SCALE/100

# Convert each SVG file to PNG
for svg_file in "$SVG_DIR"/*.svg; do
    basename=$(basename "$svg_file" .svg)
    png_file="$OUTPUT_DIR/${basename}.png"

    # Extract SVG width and height attributes from viewBox
    # viewBox format is "minX minY width height"
    viewbox=$(sed -n 's/.*viewBox="\([^"]*\)".*/\1/p' "$svg_file" | head -1)
    svg_width=$(echo "$viewbox" | awk '{print $3}')
    svg_height=$(echo "$viewbox" | awk '{print $4}')
    output_width=$(echo "scale=0; $svg_width * $SCALE / 1" | bc)
    output_height=$(echo "scale=0; $svg_height * $SCALE / 1" | bc)

    echo "Processing $basename: ${svg_width}x${svg_height} -> ${output_width}x${output_height}"

    # Convert with calculated dimensions
    rsvg-convert -w "$output_width" -h "$output_height" "$svg_file" -o "$png_file"

    if [ $? -eq 0 ]; then
        ((count++))
    else
        echo "✗ Failed to convert $basename"
    fi
done

echo "Output $count file(s)."

# Available at: https://github.com/TheWozard/fix_png
fix_png --glob "assets/tiles/*.png"

echo "Edges fixed in PNG files."
