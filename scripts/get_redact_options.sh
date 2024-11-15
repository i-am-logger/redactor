#!/usr/bin/env bash

set -e

usage() {
  echo "Usage: $0 <url> <json_file>"
  exit 1
}

if [ "$#" -ne 2 ]; then
  echo "Error: Invalid number of arguments."
  usage
fi

URL=$1
JSON_FILE=$2

curl -i -X PUT "$URL" -H "Content-Type: application/json" -d @"$JSON_FILE"
