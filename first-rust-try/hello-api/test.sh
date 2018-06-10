#!/bin/bash

curl  \
  --request POST \
  --header "Content-Type: application/json" \
  --data '{"name":"benni"}' \
  http://localhost:8000/hello
