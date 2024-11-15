#!/usr/bin/env bash

set -e

usage() {
  echo "Usage: $0 <package-name> <package-version> <port> <huggingface-hub-cache>"
  exit 1
}

if [ "$#" -ne 4 ]; then
  echo "Error: Invalid number of arguments."
  usage
fi

PACKAGE_NAME=$1
PACKAGE_VERSION=$2
PORT=$3
HUGGINGFACE_HUB_CACHE=$4

HUGGINGFACE_HUB_CACHE=$(readlink -f "$HUGGINGFACE_HUB_CACHE")

nix build .#docker
docker load < ./result
docker tag ${PACKAGE_NAME}:${PACKAGE_VERSION} ${PACKAGE_NAME}:latest
docker run -v ${HUGGINGFACE_HUB_CACHE}:/cache -p ${PORT}:${PORT} ${PACKAGE_NAME}:latest
