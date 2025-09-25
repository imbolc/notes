#!/usr/bin/env sh

cat <<EOF
Foo
Bar
EOF

# Indentation removal, works with TAB-based indent ONLY
if true; then
	cat <<-EOF
		Baz
		Spam
	EOF
fi
