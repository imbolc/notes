#!/usr/bin/env sh

# Prints in red to stderr
eprintln() {
	if [ -t 2 ] && [ "$TERM" != "dumb" ]; then
		printf '\033[0;31m%s\033[0m\n' "$1" >&2
	else
		printf '%s\n' "$1" >&2
	fi
}

eprintln "This text might be red, depending on terminal support"
