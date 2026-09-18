#!/usr/bin/env bash

set -euo pipefail

: "${SCAI_ARCHIVE:?SCAI_ARCHIVE must name the verified release archive}"
: "${SCAI_PUBLISHED:?SCAI_PUBLISHED must point to the downloaded release archive}"
: "${SCAI_REBUILT:?SCAI_REBUILT must point to the independently rebuilt archive}"
: "${SCAI_TAG:?SCAI_TAG is required}"
: "${SCAI_COMMIT:?SCAI_COMMIT is required}"
: "${SCAI_TARGET:?SCAI_TARGET is required}"
: "${SCAI_RELEASE_HOST:?SCAI_RELEASE_HOST is required}"
: "${SCAI_RELEASE_TOOLS:?SCAI_RELEASE_TOOLS is required}"
: "${SCAI_BUILD_EVIDENCE:?SCAI_BUILD_EVIDENCE must point to build evidence JSON}"
: "${SCAI_BUILD_WORKFLOW:?SCAI_BUILD_WORKFLOW is required}"
: "${SCAI_BUILD_COMMAND:?SCAI_BUILD_COMMAND is required}"

if command -v sha256sum >/dev/null 2>&1; then
  published_digest=$(sha256sum "$SCAI_PUBLISHED" | cut -d ' ' -f 1)
  rebuilt_digest=$(sha256sum "$SCAI_REBUILT" | cut -d ' ' -f 1)
else
  published_digest=$(shasum -a 256 "$SCAI_PUBLISHED" | cut -d ' ' -f 1)
  rebuilt_digest=$(shasum -a 256 "$SCAI_REBUILT" | cut -d ' ' -f 1)
fi

if [[ "$published_digest" != "$rebuilt_digest" ]]; then
  echo "The rebuilt archive does not match the published release archive" >&2
  exit 1
fi

release_uri="${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/releases/download/${SCAI_TAG}/${SCAI_ARCHIVE}"
run_uri="${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/actions/runs/${GITHUB_RUN_ID}/attempts/${GITHUB_RUN_ATTEMPT}"
workflow_path=".github/workflows/release_reproducible_build.yml"
workflow_uri="${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/blob/${GITHUB_WORKFLOW_SHA}/${workflow_path}"
release_workflow_uri="${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/blob/${SCAI_COMMIT}/.github/workflows/release_github.yml"

npx -y celq@0.7.1 \
  --arg="archive:string=$SCAI_ARCHIVE" \
  --arg="digest:string=$published_digest" \
  --arg="release_uri:string=$release_uri" \
  --arg="repository:string=${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}" \
  --arg="tag:string=$SCAI_TAG" \
  --arg="commit:string=$SCAI_COMMIT" \
  --arg="target:string=$SCAI_TARGET" \
  --arg="release_host:string=$SCAI_RELEASE_HOST" \
  --arg="release_tools:string=$SCAI_RELEASE_TOOLS" \
  --arg="release_workflow_uri:string=$release_workflow_uri" \
  --arg="build_workflow:string=$SCAI_BUILD_WORKFLOW" \
  --arg="build_command:string=$SCAI_BUILD_COMMAND" \
  --arg="workflow_name:string=$GITHUB_WORKFLOW" \
  --arg="workflow_path:string=$workflow_path" \
  --arg="workflow_ref:string=$GITHUB_WORKFLOW_REF" \
  --arg="workflow_sha:string=$GITHUB_WORKFLOW_SHA" \
  --arg="run_uri:string=$run_uri" \
  --arg="workflow_uri:string=$workflow_uri" \
  '{
    "attributes": [{
      "attribute": "REPRODUCIBLE",
      "target": {
        "name": archive,
        "uri": release_uri,
        "digest": {"sha256": digest},
        "mediaType": archive.endsWith(".zip") ? "application/zip" : "application/gzip"
      },
      "conditions": {
        "source": {
          "repository": repository,
          "tag": tag,
          "commit": commit
        },
        "comparison": {
          "algorithm": "sha256",
          "publishedDigest": digest,
          "rebuiltDigest": digest,
          "result": "identical"
        },
        "releaseBuild": {
          "workflow": release_workflow_uri,
          "host": release_host,
          "toolchain": release_tools
        },
        "reproductionBuild": {
          "workflow": build_workflow,
          "attestationWorkflow": workflow_path,
          "workflowRef": workflow_ref,
          "workflowCommit": workflow_sha,
          "host": this.host,
          "target": target,
          "command": build_command,
          "toolchain": this.toolchain,
          "predicateGenerator": "npx -y celq@0.7.1"
        }
      },
      "evidence": {
        "name": "GitHub Actions reproducible-build workflow run",
        "uri": run_uri,
        "mediaType": "text/html"
      }
    }],
    "producer": {
      "name": workflow_name,
      "uri": workflow_uri
    }
  }' < "$SCAI_BUILD_EVIDENCE" > scai-predicate.json
