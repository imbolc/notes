#!/usr/bin/env bash

set -e  # exit if a command fails
set -u  # error when referencing undefined variable

libs=(
    "ipywidgets>=8"
    "jupyterlab-vim>=4"
    "jupyterlab>=4"
    "matplotlib>=3"
    "pandas>=2"
    "scikit_learn>=1"
    "tqdm>=4"
)

venv=.venv

if [ ! -d $venv ]; then
    python3 -m venv $venv
    $venv/bin/pip install "${libs[@]}"
fi

$venv/bin/python -m jupyterlab 


