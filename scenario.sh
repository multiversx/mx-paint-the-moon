BASE_DIR=$(pwd)

# Run interactor scenarios from the microservice
cargo test --manifest-path=./microservice/Cargo.toml -- --nocapture