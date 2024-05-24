#!/bin/bash

aws kms encrypt \
	--key-id alias/test-key \
	--plaintext fileb://.env \
	--query CiphertextBlob \
	--output text \
	> .env.encrypt
