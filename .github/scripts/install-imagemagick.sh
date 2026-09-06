#!/bin/sh
set -eu

# ImageMagick dev + clang for magick_rust (bindgen) in the rust-build.action
# container (alpine). musl-dev provides the headers bindgen needs.
apk add --no-cache imagemagick-dev clang-dev pkgconf musl-dev