FROM quay.io/fedora/fedora:43 AS base
# add parallel downloads
RUN  echo "[main]" > /etc/dnf/libdnf5.conf.d/80-user-settings.conf \ 
    && echo "max_parallel_downloads=10" >> /etc/dnf/libdnf5.conf.d/80-user-settings.conf
RUN dnf update -y && dnf in glibc-langpack-th glibc-langpack-en -y
ENV TZ="Asia/Bangkok" \
    LANG=th_TH.utf8 \
    LC_ALL=th_TH.utf8


FROM base AS chef
RUN dnf group install development-tools -y \
    && dnf in git \
    cargo rustup \
    rust-src rust-std-static-wasm32-unknown-unknown \
    -y
# Setup variable environment
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:/root/.cargo/bin:$PATH 
# install cargo-chef
RUN cargo install cargo-chef
WORKDIR /app


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# install dx
RUN mkdir -p /root/.cargo/bin
RUN curl -sSL http://dioxus.dev/install.sh | sh
ENV PATH="/root/.cargo/bin:$PATH"
COPY . .
# Create the final bundle folder. Bundle always executes in release mode with optimizations enabled
RUN dx bundle --package web --web --release


FROM quay.io/fedora/fedora-minimal:43 AS runtime
WORKDIR /app
COPY --from=builder /app/target/dx/web/release/web /app/web

ENV PORT=8080
ENV IP=0.0.0.0

EXPOSE 8080

ENTRYPOINT [ "/app/web/web" ]