#!/usr/bin/env bash
set -ex

# 1. checkout the certain branch / tag
# 2. Run `cargo build to build the docs`
# 3. copy the doc folder over to `/tmp`
# 4. Now, you need to switch your substrate folder branch to gh-page
#    - copy the doc folder to this folder,
#    - if `latest` flag is added, update the `index.html`
#    - git force push back to the substrate git repo

# Example:
#   rustdocs-release.sh --help
#   rustdocs-release.sh deploy monthly-2021-10
#   rustdocs-release.sh deploy --latest monthly-2021-10
#   rustdocs-release.sh remove monthly-2021-10

# The git repo http URL
# REMOTE_REPO="https://github.com/paritytech/substrate.git"
REMOTE_REPO="https://github.com/jimmychu0807/advent-of-code-2020.git"

# tmp location that the built doc is copied to. Better be an absolute path
TMP_PREFIX="/tmp"
DOC_INDEX_PAGE="binary_boarding/index.html"
CARGO_NIGHTLY=true
GIT_REMOTE="gh-me"

# 1. checkout the certain branch / tag
# Assuming we check out from the Substrate repo
SCRIPT=$(realpath $0)
SCRIPT_PATH=$(dirname $SCRIPT)
PROJECT_PATH=$(dirname ${SCRIPT_PATH})
PROJECT_NAME=$(basename "$PROJECT_PATH")

SUBCMD=$1
BUILD_RUSTDOC_REF=$2
LATEST=true

# Check there is no local changes before proceeding
[[ -n $(git status --porcelain) ]] && echo "Local changes exist, please either discard or commit them as this command will change the current checkout branch." && exit 0

CURRENT_GIT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
TMP_PROJECT_PATH="${TMP_PREFIX}/${PROJECT_NAME}"
DOC_PATH="${TMP_PROJECT_PATH}/${BUILD_RUSTDOC_REF}"

rm -rf "${TMP_PROJECT_PATH}" && mkdir "${TMP_PROJECT_PATH}"

# Copy .gitignore file to tmp
[[ -e "${PROJECT_PATH}/.gitignore" ]] && cp "${PROJECT_PATH}/.gitignore" "${TMP_PROJECT_PATH}"

git fetch --all
git checkout -f $BUILD_RUSTDOC_REF || { echo "Checkout ${BUILD_RUSTDOC_REF} error." && exit 0; };

# Build the docs
time cargo $($CARGO_NIGHTLY && echo "+nightly") doc --no-deps --workspace --all-features --verbose || \
  { echo "Generate $BUILD_RUSTDOC_REF rustdocs failed" && exit 0; }
rm -f target/doc/.lock

# Moving the built doc to the tmp location
mv target/doc "${DOC_PATH}"

[[ -n "${DOC_INDEX_PAGE}" ]] && \
  echo "<meta http-equiv=refresh content=0;url=${DOC_INDEX_PAGE}>" > "${DOC_PATH}/index.html"

# git checkout `gh-pages` branch
git fetch "${GIT_REMOTE}" gh-pages

git checkout gh-pages
# Move the built back
[[ -e "${TMP_PROJECT_PATH}/.gitignore" ]] && cp -f "${TMP_PROJECT_PATH}/.gitignore" .
# Ensure the destination dir doesn't exist under current path.
rm -rf "${BUILD_RUSTDOC_REF}"
mv -f "${DOC_PATH}" "${BUILD_RUSTDOC_REF}"

# -- Run `index-tpl-crud` to update the index.html
# Check if `index-tpl-crud` exists
which index-tpl-crud &> /dev/null || yarn global add @jimmychu0807/index-tpl-crud
index-tpl-crud upsert $($LATEST && echo "-l") ./index.html "$BUILD_RUSTDOC_REF"

# Add the symlink
$LATEST && rm -rf latest && ln -sf "${BUILD_RUSTDOC_REF}" latest
# Upload files
git add --all
git commit -m "___Updated docs for ${BUILD_RUSTDOC_REF}___" || echo "___Nothing to commit___"
git push "${GIT_REMOTE}" gh-pages --force

# Remove the tmp asset created
rm -rf "${TMP_PROJECT_PATH}"

# Resume back previous checkout branch.
git checkout -f "$CURRENT_GIT_BRANCH"
