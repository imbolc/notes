#!/usr/bin/env sh

theme="${1-}"
if [ "$theme" != "dark" ] && [ "$theme" != "light" ]; then
    printf '\033[0;31m%s\033[0m\n' "Error: Invalid argument" >&2
    echo "Usage: $0 [dark|light]" >&2
    exit 1
fi

if ! [ "$2" -eq "$2" ] 2>/dev/null || [ "$2" -lt 8 ] || [ "$2" -gt 19000 ]; then
    echo "Error: First argument must be an integer between 8 and 19000" >&2
    exit 1
fi
