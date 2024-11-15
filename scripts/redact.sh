#!/usr/bin/env bash

set -e

usage() {
  echo "Usage: $0 <url> <text_file>"
  exit 1
}

if [ "$#" -ne 2 ]; then
  echo "Error: Invalid number of arguments."
  usage
fi

URL=$1
TEXT_FILE=$2

curl -i -X POST "$URL" -H "Content-Type: text/plain" --data-binary @"$TEXT_FILE"
