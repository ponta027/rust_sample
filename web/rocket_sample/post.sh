#!/bin/bash

HOST=localhost
PORT=9000

curl -X POST -H "Content-Type:application/json" -d '{"message":"HOGE"}' http://${HOST}:${PORT}/json

curl http://${HOST}:${PORT}/json/0

curl -X PUT -H "Content-Type:application/json" -d '{"message":"HOGE2"}' http://${HOST}:${PORT}/json/0

curl http://${HOST}:${PORT}/json/0
