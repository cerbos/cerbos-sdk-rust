buf curl --schema buf.build/cerbos/cloud-api --header "x-cerbos-auth: $TOKEN" -d "{ \"store_id\": \"$CERBOS_HUB_STORE_ID\" }" \
${CERBOS_HUB_API_ENDPOINT}/cerbos.cloud.store.v1.CerbosStoreService/ListFiles

