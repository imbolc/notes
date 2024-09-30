#!/usr/bin/env sh

# will disappear after the script exit
foo='a local variable'

# will be left accessible in the current shell session
export FOO='a global variable'

# if variable `bar` is empty assign it a default value
${foo:='a default value'}

# positional arguments
echo "first argument is '$1'"

echo "foo='$foo'"
echo "FOO='$FOO'"

for i in 1 2 3 4 5; do
    echo "i = $i"
done
