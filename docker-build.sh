#!env bash
set -e

BIN_NAME="v2parser"
TARGET_TRIPLE="x86_64-unknown-linux-gnu"
HOST_TARGET_DIR="$(pwd)/target/$TARGET_TRIPLE/release"

docker build \
  --platform=linux/amd64 \
  -t rust-builder-temp \
  -f Dockerfile .

container_id=$(docker create rust-builder-temp)

mkdir -p ./target

docker cp "$container_id:/app/target/$TARGET_TRIPLE/release/$BIN_NAME" "./$BIN_NAME"
docker rm "$container_id"

chmod +x "./$BIN_NAME"
echo "✅ Built binary: ./$BIN_NAME"
