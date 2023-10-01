FROM rust:1.72-buster AS rust

COPY database/hibp.bloom hibp.bloom
COPY database/last-updated.txt last-updated.txt
COPY src src
COPY easypwned/easypwned_bloom easypwned/easypwned_bloom
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo install --path .
RUN hashcheck --version

CMD hashcheck
