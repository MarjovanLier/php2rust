# The image CI runs the tests in on the self-hosted runner: one PHP version (P2R-77), the
# Rust toolchain that rust-toolchain.toml pins (P2R-83) and Composer for the harness's
# Laravel packages (P2R-608). The workflow builds it with PHP and RUST as arguments.
ARG PHP=8.5
FROM composer:2.10.3 AS composer

FROM php:${PHP}-cli
# git and unzip for actions and Composer, curl for rustup, and mbstring, which Laravel needs
# and the 8.6 image leaves out.
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl git unzip libonig-dev \
    && if ! php -m | grep -qx mbstring; then docker-php-ext-install mbstring; fi \
    && rm -rf /var/lib/apt/lists/*
COPY --from=composer /usr/bin/composer /usr/local/bin/composer
ENV RUSTUP_HOME=/usr/local/rustup CARGO_HOME=/usr/local/cargo PATH=/usr/local/cargo/bin:$PATH
ARG RUST
RUN curl -fsSL https://sh.rustup.rs | sh -s -- -y --no-modify-path --profile minimal \
        --default-toolchain "${RUST}" -c rustfmt -c clippy \
    && rustc --version
