# Docker file for deployment.
#
# Downloads the bloom data file from S3.
FROM rust:1.72-buster AS rust

RUN curl -OL https://s3.ap-southeast-1.amazonaws.com/hibp.saveoursecrets.com/last-updated.txt
RUN curl -OL https://s3.ap-southeast-1.amazonaws.com/hibp.saveoursecrets.com/hibp.bloom
COPY src src
COPY easypwned/easypwned_bloom easypwned/easypwned_bloom
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN cargo install --path .
RUN hashcheck --version

CMD hashcheck
