#!/usr/bin/env sh

# Parameters are passed through $1, $2, ...
greet() {
	if [ -z "$1" ]; then
		return 1 # Return error code 1 if no argument is provided
	fi
	printf "Hello, %s!" "$1"
}

# Passing all the parameter to another function with $@
decorated_greet() {
	echo "=== $(greet "$@") ==="
}

decorated_greet Alice        # === Hello, Alice! ===
greet || echo "NoName error" # NoName error
