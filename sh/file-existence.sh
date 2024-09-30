#!/usr/bin/env sh

# Check file existence

# `-f` - for a file
# `-d` - for a directory
# `-e` - for any filesystem object (file, directory, symlink, ...)

# existence of a file
if [ -f "/foo.txt" ]; then
    echo "File exists"
fi

# non-existence of a folder
if [ ! -d "/my-folder" ]; then
    echo "Directory doesn't exist"
fi
