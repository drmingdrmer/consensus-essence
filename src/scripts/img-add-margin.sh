#!/bin/sh

# Add 50% margin to left and right side of an image
# Usage:
#   $0 <fn>.<suffix>
# output to:
#   <fn>-margin.<suffix>

fn="${1}"

suffix="${fn##*.}"
output_fn="${fn%.*}-margin.$suffix"

convert -resize 800 -gravity center -extent 1600 "$fn" "$output_fn"
