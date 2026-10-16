#!/usr/bin/env bash
set -euo pipefail

# Run from the directory as the repository root.
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

# Check for the --dry-run flag (only flag we plan on supporting).
DRY_RUN=false
if [[ ${1-} == --dry-run ]]; then
    DRY_RUN=true
elif [[ $# -gt 0 ]]; then
    echo "usage: $0 [--dry-run]" >&2
    exit 2
fi

# Check that we're on the correct git branch, and up-to-date.
if [[ $(git branch --show-current) != main ]]; then
    echo "error: the git repo is not on the 'main' branch, which is required for a release" >&2
    exit 1
fi
git fetch --quiet
if [[ $(git rev-list --count HEAD..@{upstream}) -ne 0 ]]; then
    echo "error: the local branch is behind the remote branch - please pull first" >&2
    exit 1
fi
if [[ $(git rev-list --count @{upstream}..HEAD) -ne 0 ]]; then
    echo "error: the local branch is ahead of the remote branch - please push first" >&2
    exit 1
fi

# Check that only certain allowed files are changed.
CHANGES=$(git status --porcelain --untracked-files=all)
if [[ -n ${CHANGES-} && ${CHANGES-} != " M Cargo.toml" && ${CHANGES-} != "M  Cargo.toml" ]]; then
    echo "error: files other than Cargo.toml have been modified - please commit and push them separately" >&2
    exit 1
fi

# Get release metadata for comparison purposes.
CURRENT_VERSION=$(
    curl -fsS \
    -H 'User-Agent: typed-ident release script' \
    "https://crates.io/api/v1/crates/typed-ident" |
    jq -er '.crate.max_version'
)
TARGET_VERSION=$(
    cargo metadata --no-deps --format-version 1 |
    jq -er '.packages[] | select(.name == "typed-ident") | .version'
)
if [[ ! ${CURRENT_VERSION-} =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "error: the current version doesn't match our format expectations ($CURRENT_VERSION)" >&2
    exit 1
fi
if [[ ! ${TARGET_VERSION-} =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "error: the target version doesn't match our format expectations ($TARGET_VERSION)" >&2
    exit 1
fi
if [[ ${CURRENT_VERSION-} == ${TARGET_VERSION-} ]]; then
    echo "error: current published version ($CURRENT_VERSION) is the same as the target version ($TARGET_VERSION)" >&2
    echo "       did you maybe forget to update Cargo.toml?" >&2
    exit 1
fi
if [[ ! $(printf '%s\n%s\n' "$CURRENT_VERSION" "$TARGET_VERSION" | sort -V | tail -n 1) == "$TARGET_VERSION" ]]; then
    echo "error: target version ($TARGET_VERSION) must be newer than published version ($CURRENT_VERSION)" >&2
    exit 1
fi
TARGET_TAG="v$TARGET_VERSION"

# Check the tag was not already created before.
if git rev-parse -q --verify "refs/tags/$TARGET_TAG" >/dev/null; then
    echo "error: the target tag ($TARGET_TAG) already exists - did something go wrong?" >&2
    exit 1
fi

# First-pass allowing a dirty git repo - we'll commit soon.
./scripts/test.sh
cargo package -p typed-ident --list --allow-dirty
cargo package -p typed-ident --allow-dirty

# Commit and create the tag for the new release.
if [[ -n ${CHANGES-} ]]; then
    if [[ $DRY_RUN == true ]]; then
        echo "dry run enabled - skipping git commit step"
    else
        git add Cargo.toml
        git diff --cached --check
        git commit -m "Release $TARGET_TAG"
    fi
fi

# Second-pass disallowing a dirty git repo - we're getting ready for publish
if [[ $DRY_RUN == true ]]; then
    cargo package -p typed-ident --list --allow-dirty
    cargo package -p typed-ident --allow-dirty
    cargo publish -p typed-ident --dry-run
    echo "dry run enabled - skipping publish step"
    exit 0
else
    cargo package -p typed-ident --list
    cargo package -p typed-ident
    cargo publish -p typed-ident --dry-run
fi

# Actually perform the publish operation
git tag -s "$TARGET_TAG" -m "Release $TARGET_TAG"
git tag -v "$TARGET_TAG"
cargo publish -p typed-ident
git push origin main
git push origin "$TARGET_TAG"
