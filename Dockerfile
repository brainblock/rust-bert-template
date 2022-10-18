FROM quay.io/brainblock/rust-torch-cuda116 as builder

COPY . .

RUN cargo build --release





FROM debian:bullseye as cosine_similarity

# torch libraries
COPY --from=builder /usr/local/libtorch/lib/* /usr/lib/

# local models
COPY --from=builder ./models models

# binary files build
COPY --from=builder ./target/release/rust-bert-example rust-bert-example

ENTRYPOINT [ "/rust-bert-example" ]
