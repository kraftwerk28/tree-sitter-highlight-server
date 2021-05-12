FROM rust:1.51
WORKDIR /opt/app
COPY ./ ./
RUN cargo build --release
CMD ["cargo", "run", "--release"]
