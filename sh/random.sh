#!/usr/bin/env sh

random_1_to_10=$(od -An -N1 -tu1 /dev/urandom | awk '{print ($1 % 10) + 1}')
echo "$random_1_to_10"
