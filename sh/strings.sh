#!/usr/bin/env sh

set -x

# Single quotsess - nothing expands
echo '$USER\t$(date)' # $USER\t$(date)

# Double quotes - everything expands
echo "$USER\t$(date +%Y)" # imbolc  2024
