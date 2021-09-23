#!/bin/bash

curl -X POST -H "Content-Type:application/json" -d '{"message":"HOGE"}' http://localhost:8000/json

curl http://localhost:8000/json/0

curl -X PUT -H "Content-Type:application/json" -d '{"message":"HOGE2"}' http://localhost:8000/json/0

curl http://localhost:8000/json/0
