#!/bin/bash

aws kms decrypt \
	--key-id alias/test-key \
	--ciphertext-blob fileb://<(base64 --decode .env.encrypt) \
	--output text \
	--query Plaintext \
	--no-cli-pager \
	| base64 --decode \
	> .env
