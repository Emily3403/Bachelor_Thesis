#!/usr/bin/env bash

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
cd "$SCRIPT_DIR/.." || exit 1

pdflatex -ini -jobname="template" "&pdflatex" mylatexformat.ltx "main.tex"
rm template.log
