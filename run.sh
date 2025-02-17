#!/bin/bash

export ENV_OUTPUT_DIR_NAME=./sample.d/out.d
export ENV_MAX_LINE_PER_FILE=10
export ENV_FILE_SYNC_TYPE=nop
export ENV_SHOW_PROGRESS=true

mkdir -p "${ENV_OUTPUT_DIR_NAME}"

seq 1 100 |
	./rs-split
