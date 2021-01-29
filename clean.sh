#!/bin/sh

find . -maxdepth 1 -type f -not -name "*.sh" -executable -delete
