#!/usr/bin/env bash
set -x

# 1. checkout the certain branch / tag
# 2. Run `cargo build to build the docs`
# 3. copy the doc folder over to `/tmp`
# 4. Now, you need to switch your substrate folder branch to gh-page
#    - copy the doc folder to this folder,
#    - if `latest` flag is added, update the `index.html`
#    - git force push back to the substrate git repo

# Example:
#   rustdocs-release.sh --help
#   rustdocs-release.sh deploy monthly-2021-12
#   rustdocs-release.sh deploy --latest monthly-2021-12
#   rustdocs-release.sh remove monthly-2021-12

# The git repo http URL
# REMOTE_REPO="https://github.com/paritytech/substrate.git"
REMOTE_REPO="https://github.com/jimmychu0807/advent-of-code-2020.git"

# tmp location that the built doc is copied to. Better be an absolute path
TMP_DOC_PREFIX="/tmp"
DOC_INDEX_PAGE="sc_service/index.html"
CARGO_NIGHTLY=true

# 1. checkout the certain branch / tag
# Assuming we check out from the Substrate repo
SCRIPT=$(realpath $0)
SCRIPT_PATH=$(dirname $SCRIPT)
PROJECT_DIR=$(dirname ${SCRIPT_PATH})

SUBCMD=$1
BUILD_RUSTDOC_REF=$2

git fetch --all
git checkout -f $BUILD_RUSTDOC_REF || { echo "Checkout ${BUILD_RUSTDOC_REF} error." && exit 0; };

time cargo $($CARGO_NIGHTLY && echo "+nightly") doc --no-deps --workspace --all-features --verbose || \
  { echo "Generate $BUILD_RUSTDOC_REF rustdocs failed" && exit 0; }

rm -f target/doc/.lock

TMP_DOC_PATH="${TMP_DOC_PREFIX}/${PROJECT_DIR}"
rm -rf "${TMP_DOC_PATH}"
mkdir "${TMP_DOC_PATH}"
mv target/doc "${TMP_DOC_PATH}/${BUILD_RUSTDOC_REF}"

[[ -z "${DOC_INDEX_PAGE}" ]] && \
  echo "<meta http-equiv=refresh content=0;url=${DOC_INDEX_PAGE}>" > "${TMP_DOC_PATH}/${BUILD_RUSTDOC_REF}/index.html"

popd
