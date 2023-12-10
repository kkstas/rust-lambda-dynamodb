#!/bin/bash

HTTP_ENDPOINT=$1

curl -X POST "$HTTP_ENDPOINT/db" \
    --header 'Content-Type: application/json' \
    --data '{ "p_type": "user", "age": "55", "username": "suck_my_big_smoke", "first": "Carl", "last": "Johnson"}'
