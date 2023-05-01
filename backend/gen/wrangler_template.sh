#!/bin/bash

# Load env
export $(cat .env)
# Replace env in wrangler template
envsubst '$ACCOUNT_ID $KV_BINDING $KV_ID $KV_PREVIEW_ID' \
  < ./gen/wrangler.toml.template \
  > wrangler.toml

# Delete CR
tr -d '\r' < wrangler.toml > wrangler_temp.toml
mv wrangler_temp.toml wrangler.toml
