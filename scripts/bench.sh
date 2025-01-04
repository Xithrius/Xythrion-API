#!/usr/bin/env bash

ab -n 1000 -c 10 localhost:8080/health
