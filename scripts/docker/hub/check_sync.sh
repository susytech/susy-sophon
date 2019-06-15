#!/bin/bash
# checks if susy has a fully synced blockchain

SOF_SYNCING=$(curl -X POST --data '{"jsonrpc":"2.0","method":"sof_syncing","params":[],"id":1}' http://localhost:8545 -H 'Content-Type: application/json')
RESULT=$(echo "$SOF_SYNCING" | jq -r .result)

if [ "$RESULT" == "false" ]; then
  echo "Susy is ready to start accepting traffic"
  exit 0
else
  echo "Susy is still syncing the blockchain"
  exit 1
fi
