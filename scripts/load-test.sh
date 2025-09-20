#!/usr/bin/env bash
set -euo pipefail
docker compose up -d
docker run --rm -i --network host \
  -v "$PWD/k6:/k6" \
  grafana/k6 run /k6/script.js
  
