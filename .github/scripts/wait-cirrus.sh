#!/bin/bash
set -e

# Wait for a Cirrus CI build to complete by polling its status

if [ -z "$CIRRUS_TOKEN" ]; then
  echo "Error: CIRRUS_TOKEN environment variable is not set"
  exit 1
fi

if [ -z "$BUILD_ID" ]; then
  echo "Error: BUILD_ID environment variable is not set"
  exit 1
fi

TIMEOUT=${TIMEOUT:-900}  # Default 15 minutes
INTERVAL=${INTERVAL:-30} # Default 30 seconds
ELAPSED=0

echo "Waiting for build $BUILD_ID to complete..."
echo "Timeout: ${TIMEOUT}s, Check interval: ${INTERVAL}s"

while [ $ELAPSED -lt $TIMEOUT ]; do
  STATUS_RESPONSE=$(curl -s -X POST \
    -H "Authorization: Bearer $CIRRUS_TOKEN" \
    -H "Content-Type: application/json" \
    "https://api.cirrus-ci.com/graphql" \
    -d "{
      \"query\": \"query { build(id: \\\"$BUILD_ID\\\") { status } }\"
    }")
  
  STATUS=$(echo "$STATUS_RESPONSE" | jq -r '.data.build.status')
  echo "[${ELAPSED}s] Current status: $STATUS"
  
  case $STATUS in
    COMPLETED)
      echo "✓ Build completed successfully!"
      exit 0
      ;;
    FAILED)
      echo "✗ Build failed"
      exit 1
      ;;
    ABORTED)
      echo "✗ Build was aborted"
      exit 1
      ;;
    ERRORED)
      echo "✗ Build encountered an error"
      exit 1
      ;;
    EXECUTING|CREATED|QUEUED|TRIGGERED)
      # Build is still in progress
      sleep $INTERVAL
      ELAPSED=$((ELAPSED + INTERVAL))
      ;;
    null)
      echo "✗ Build not found or invalid response"
      echo "Response: $STATUS_RESPONSE"
      exit 1
      ;;
    *)
      echo "✗ Unknown status: $STATUS"
      echo "Response: $STATUS_RESPONSE"
      exit 1
      ;;
  esac
done

echo "✗ Timeout: Build did not complete within ${TIMEOUT} seconds"
exit 1