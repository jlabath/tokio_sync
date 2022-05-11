#!/bin/sh
siege --time=60s -c 1 -b http://127.0.0.1:3030/sync_sleep
