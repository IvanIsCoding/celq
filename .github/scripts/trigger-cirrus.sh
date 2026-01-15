#!/bin/bash
set -e

# Trigger a Cirrus CI build via GraphQL API and output the build ID

if [ -z "$CIRRUS_TOKEN" ]; then
  echo "Error: CIRRUS_TOKEN environment variable is not set"
  exit 1
fi

if [ -z "$GITHUB_REPOSITORY_OWNER" ] || [ -z "$GITHUB_REPOSITORY_NAME" ] || [ -z "$GITHUB_REF_NAME" ] || [ -z "$GITHUB_SHA" ]; then
  echo "Error: Required GitHub environment variables are not set"
  exit 1
fi

echo "Getting repository ID for $GITHUB_REPOSITORY_OWNER/$GITHUB_REPOSITORY_NAME..."

# Get repository ID first
REPO_RESPONSE=$(curl -s -X POST \
  -H "Authorization: Bearer $CIRRUS_TOKEN" \
  -H "Content-Type: application/json" \
  "https://api.cirrus-ci.com/graphql" \
  -d "{
    \"query\": \"query { ownerRepository(platform: \\\"github\\\", owner: \\\"$GITHUB_REPOSITORY_OWNER\\\", name: \\\"$GITHUB_REPOSITORY_NAME\\\") { id } }\"
  }")

REPO_ID=$(echo "$REPO_RESPONSE" | jq -r '.data.ownerRepository.id')

if [ -z "$REPO_ID" ] || [ "$REPO_ID" = "null" ]; then
  echo "Error: Failed to get repository ID"
  echo "Response: $REPO_RESPONSE"
  exit 1
fi

echo "Repository ID: $REPO_ID"

# Trigger the build using GraphQL mutation
echo "Triggering build for branch $GITHUB_REF_NAME at commit $GITHUB_SHA..."

# Generate a unique client mutation ID
CLIENT_MUTATION_ID="github-actions-$(date +%s)-$$"

BUILD_RESPONSE=$(curl -s -X POST \
  -H "Authorization: Bearer $CIRRUS_TOKEN" \
  -H "Content-Type: application/json" \
  "https://api.cirrus-ci.com/graphql" \
  -d "{
    \"query\": \"mutation { createBuild(input: { clientMutationId: \\\"$CLIENT_MUTATION_ID\\\", repositoryId: \\\"$REPO_ID\\\", branch: \\\"$GITHUB_REF_NAME\\\", sha: \\\"$GITHUB_SHA\\\" }) { build { id } } }\"
  }")

BUILD_ID=$(echo "$BUILD_RESPONSE" | jq -r '.data.createBuild.build.id')

if [ -z "$BUILD_ID" ] || [ "$BUILD_ID" = "null" ]; then
  echo "Error: Failed to create build"
  echo "Response: $BUILD_RESPONSE"
  exit 1
fi

echo "Build ID: $BUILD_ID"
echo "build_id=$BUILD_ID" >> "$GITHUB_OUTPUT"