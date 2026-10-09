#!/usr/bin/env bash

set -euo pipefail

if [ "$#" -ne 1 ]; then
    echo "usage: $0 <docker-image>"
    echo "example:"
    echo "  $0 ghcr.io/somedude/busymonkey:a81c327"
    exit 1
fi

IMAGE="$1"

if ! command -v docker >/dev/null 2>&1; then
    echo "error: Docker is not installed."
    exit 1
fi

if ! docker compose version >/dev/null 2>&1; then
    echo "error: Docker Compose v2 is not available."
    exit 1
fi

if [ ! -f ".env" ]; then
    echo "error: .env not found."
    echo "create it from .env.example first:"
    echo "  cp .env.example .env"
    exit 1
fi

if [ ! -f "config.toml" ]; then
    echo "error: config.toml not found."
    echo "create it from config.toml.example first:"
    echo "  cp config.toml.example config.toml"
    exit 1
fi

export BUSYMONKEY_IMAGE="$IMAGE"

echo "validating Docker Compose configuration..."
docker compose --env-file .env config --quiet

echo "pulling image:"
echo "  $BUSYMONKEY_IMAGE"
docker compose --env-file .env pull app

echo "starting services..."
docker compose --env-file .env up \
    -d \
    --no-build \
    --wait

echo
echo "deploted successfully :3"
docker compose --env-file .env ps
