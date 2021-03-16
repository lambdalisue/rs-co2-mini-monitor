#!/bin/bash
TARGET="$1"

cd $(dirname $0)

if [[ -d "$TARGET" ]]; then
  cd "$TARGET"
  docker build -t "$TARGET:local" .
fi
