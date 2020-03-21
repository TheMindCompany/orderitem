#!/bin/bash

cargo build --release
cp target/release/orderitem darwin
rm -rf target

docker build -t buildercontainer:debian -f docker/Dockerfile.debian .
docker create --name=buildercontainer-debian buildercontainer:debian
docker cp buildercontainer-debian:/opt/orderitem/target/release/orderitem debian
docker rm buildercontainer-debian
