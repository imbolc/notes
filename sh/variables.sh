#!/usr/bin/env sh

first_arg="${1:-default first arg}"

# will disappear after the script exit
foo='a local variable'

# will be left accessible in the current shell session
export FOO='a global variable'

# positional arguments
echo "first argument is $first_arg"

echo "foo='$foo'"
echo "FOO='$FOO'"

for i in 1 2 3 4 5; do
    echo "i = $i"
done
