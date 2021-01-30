#!/bin/sh

do_test() {
    rustc --out-dir build/ src/$1.rs

    NLINES=$(wc -l db/testcases.txt | sed "s/ .*//")
    SLINE=$(grep -n "$1" db/testcases.txt | sed "s/:.*//")
    ELINE=$(tail -n $((NLINES - SLINE)) db/testcases.txt | grep -n "^$" | sed "s/:.*//" | head -n 1)
    TEST_DATA=$(sed -n ${SLINE},$((ELINE + SLINE - 1))p db/testcases.txt)
    DATA_LEN=$(echo "$TEST_DATA" | wc -l)
    NUM_TESTS=$(echo "$TEST_DATA" | grep "test[0-9]\+" | wc -w)
    TEST_NUM=1
    while [ $TEST_NUM -le $NUM_TESTS ]; do
        echo -n "Running test ${1}${TEST_NUM}... "
        TEST_LINE=$(echo "$TEST_DATA" | grep -n "test${TEST_NUM}" | sed "s/:.*//")
        TEST_END=$(echo "$TEST_DATA" | grep -n "test$((TEST_NUM + 1))" | sed "s/:.*//")
        if [ -z $TEST_END ]; then
            TEST_END=$DATA_LEN
        else
            TEST_END=$((TEST_END - 1))
        fi
        TEST_ENTRY=$(echo "$TEST_DATA" | sed -n ${TEST_LINE},${TEST_END}p)
        ENTRY_LEN=$(echo "$TEST_ENTRY" | wc -l)
        EXPECTED_LINE=$(echo "$TEST_ENTRY" | grep -n "expected" | sed "s/:.*//")
        INPUT_DATA=$(echo "$TEST_ENTRY" | sed -n 3,$((EXPECTED_LINE - 1))p)
        OUTPUT=$(echo "$INPUT_DATA" | ./build/$1)
        EXPECTED=$(echo "$TEST_ENTRY" | sed -n $((EXPECTED_LINE + 1)),${ENTRY_LEN}p)
        if [ "$OUTPUT" == "$EXPECTED" ]; then
            echo OK
        else
            echo FAILED
            echo got:
            echo "$OUTPUT"
            echo expected:
            echo "$EXPECTED"
        fi
        TEST_NUM=$((TEST_NUM + 1))
    done
}

if [ ! -z $1 ]; then
    do_test $1
else
    while read line; do
        do_test $line
    done < db/entries.txt
fi
