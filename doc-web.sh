#!/bin/sh

port=20680
addr=127.0.0.1
docd=./target

miniserve \
	--interfaces "${addr}" \
	--port ${port} \
	"${docd}"
