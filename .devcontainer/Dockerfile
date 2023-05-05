FROM rust:1-bullseye

# アップデートなど
RUN apt update && apt upgrade -y  \
    && apt install -y tree nodejs \
    && apt clean \
    && rm -rf /var/lib/apt/lists/*

# starshipの導入
RUN curl -sS https://starship.rs/install.sh | sh -s -- --yes && \
    echo eval "$(starship init bash)" >> ~/.bashrc

# Rust便利ツールの追加
RUN cargo install cargo-edit cargo-watch  \
    && DEBIAN_FRONTEND=noninteractive cargo install cargo-compete \
    && rustup component add rustfmt clippy rls rust-analysis rust-src \
    &&  rustup toolchain install 1.42.0-aarch64-unknown-linux-gnu

