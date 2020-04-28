#!/bin/bash

vegeta  -cpus 1 attack -rate 500   -duration=10s  -targets target.list | vegeta report -type=json | jq '.' > metrics.json

