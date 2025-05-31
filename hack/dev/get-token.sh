#!/bin/sh

export TOKEN=$(curl -s -X POST https://api.stg-spitfire.cerbos.tech/cerbos.cloud.apikey.v1.ApiKeyService/IssueAccessToken \
  -H "Content-Type: application/json" \
  -d "{
    \"client_id\": \"$CERBOS_HUB_CLIENT_ID\",
    \"client_secret\": \"$CERBOS_HUB_CLIENT_SECRET\"
  }" | jq -r '.accessToken')
