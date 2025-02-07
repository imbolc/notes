#!/usr/bin/env sh

# `[` - is actually a synonym for built-in `test` utility with required closing `]`

# Numbers
rnd=$(od -An -N1 -tu1 /dev/urandom | awk '{print ($1 % 7) - 3}') # random number from -3 to 3

if [ "$rnd" -lt 0 ]; then
    echo "$rnd < 0"
elif [ "$rnd" = 0 ]; then
    echo "$rnd is 0"
else
    echo "$rnd > 0"
fi
