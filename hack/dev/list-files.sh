curl -s https://api.stg-spitfire.cerbos.tech/cerbos.cloud.store.v1.CerbosStoreService.ListFiles/ \
  -H "Content-Type: application/json" -H "x-cerbos-auth: $TOKEN" \
  -d "{ \"store_id\": \"$CERBOS_HUB_STORE_ID\" }"

