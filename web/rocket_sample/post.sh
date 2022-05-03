#!/bin/bash

PORT=9000

curl -X POST -H "Content-Type:application/json" -d '{"message":"HOGE"}' http://192.168.0.30:${PORT}/json

curl http://192.168.0.30:${PORT}/json/0

curl -X PUT -H "Content-Type:application/json" -d '{"message":"HOGE2"}' http://192.168.0.30:${PORT}/json/0

curl http://192.168.0.30:${PORT}/json/0
