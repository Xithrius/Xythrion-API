#!/usr/bin/env bash

ab -n 1000000 -c 100 localhost:8001/api/link_maps/converters/all
