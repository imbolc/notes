#!/usr/bin/env bash
read -p "Are you sure (y/N)? " -n 1 -r; echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo Cancelled; exit 1
fi
