#!/usr/bin/env bash
cd "$(dirname "$0")" || exit 1
export GDK_BACKEND=x11
exec dx serve --platform desktop
