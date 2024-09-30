#!/usr/bin/env sh

printf "Press Enter to continue..."
read -r _

confirm() {
	printf "%s (y/N)? " "${1}"
	read -r __confirm_reply
	case "$__confirm_reply" in
	[Yy]) return 0 ;;
	*) return 1 ;;
	esac
}

confirm_or_exit() {
	printf "%s (y/N)? " "${1}"
	read -r __confirm_reply
	case "$__confirm_reply" in
	[Yy]) ;;
	*) exit 1 ;;
	esac
}

if confirm "Proceed"; then
	echo "Proceeded"
else
	echo "Cancelled"
	exit 1
fi

confirm_or_exit "Confirm or exit"
echo "Confirmed"
