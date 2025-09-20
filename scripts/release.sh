#!/usr/bin/env bash
set -euo pipefail
version=$(git describe --tags --match 'v*')
git tag -a "$version" -m "release $version"
git push origin "$version"
