#!/bin/sh

# Make the rust file from the template
cp src/template.rs src/$1.rs
# Add an entry for it with just the name
echo "$1" >> db/entries.txt
# Setup testcases for it
echo "${1}:" >> db/testcases.txt

# Testcase number
i=1
echo "Paste the inputs for a testcase, ending with an empty line"
while true; do
    # Black box lmao, I already mostly don't know how this works
    echo "test${i}:" >> db/testcases.txt
    echo "input:" >> db/testcases.txt
    if [ ! -z "$TMP" ]; then
        echo "$TMP" >> db/testcases.txt
    fi
    while true; do
        read TEST_DATA
        if [ -z "$TEST_DATA" ]; then
            break
        fi
        echo "$TEST_DATA" >> db/testcases.txt
    done

    echo "Paste the expected output for the testcase, ending with an empty line"

    echo "expected:" >> db/testcases.txt
    while true; do
        read TEST_DATA
        if [ "$TEST_DATA" == "" ]; then
            break
        fi
        echo "$TEST_DATA" >> db/testcases.txt
    done

    echo "Paste another testcase (or an empty line to finish)"
    read TMP
    if [ -z "$TMP" ]; then
        break
    fi
    i=$((i+1))
done

# Write a newline to show the end of this programs tests
echo >> db/testcases.txt
