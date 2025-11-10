#!/bin/bash

set -e

HOST="127.0.0.1"
PORT="4242"

for i in {1..100000}; do
    echo 'ADD "counter"'
done | nc $HOST $PORT > /dev/null

# To normal access
# nc 127.0.0.1 4242
#
# To run this in 10 different progress
# seq 10 | parallel -j 10 ./test.sh
