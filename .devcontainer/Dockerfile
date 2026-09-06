FROM mcr.microsoft.com/devcontainers/rust:1-trixie

# ImageMagick 7 dev libraries for magick_rust (needs MagickWand >= 7.1.1,
# which trixie provides; bookworm only has 6.9).
# libclang is required by bindgen, which magick_rust uses in its build script.
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        libmagickwand-dev \
        libclang-dev \
        clang \
        pkg-config \
    && rm -rf /var/lib/apt/lists/*
