#!/usr/bin/env bash

# ls -la ./target/aarch64-apple-darwin/debug
# ls -la ./target/x86_64-apple-darwin/debug
set -ex

echo "RELEASE_COMMIT: =$RELEASE_COMMIT="

build_alfred_bundle() {
    src=$1
    stage=$2
    test -f Cargo.lock || cargo generate-lockfile

    # TODO Update this to build the artifacts that matter to you
    # cross rustc --bin pinion --target "$TARGET" --release -- -C lto

    # echo "Copying executable to workflow's folder..."
    cp "pinion" "$stage"

    # Ship exactly what git tracks under res/workflow. The old blanket
    # `cp res/workflow/*` also swept up whatever happened to be sitting in that
    # directory — including, if you had ever run a local build, a 4 MB binary and
    # the previous .alfredworkflow nested inside the new one.
    git -C "$src" ls-files -z res/workflow | while IFS= read -r -d '' f; do
        cp "$src/$f" "$stage"
    done

    # echo "Creating the workflow bundle..."
    cd "$stage" || exit
    rm -f Pinion.alfredworkflow

    zip -r Pinion.alfredworkflow ./*

    # Deliberately unversioned: it makes
    #   releases/latest/download/Pinion.alfredworkflow
    # a permanent download link. The release tag already carries the version.
    mv ./Pinion.alfredworkflow "$src"/Pinion.alfredworkflow
    cd "$src"

}

src="$GITHUB_WORKSPACE"
stage=$(mktemp -d -t tmp)

echo "$GITHUB_WORKSPACE == $GITHUB_REF_NAME"
if [[ "$RELEASE_COMMIT" == "true" ]]; then
  build_type=release
else
  build_type=debug
fi
ls -lh ./target/aarch64-apple-darwin/"$build_type"/pinion
ls -lh ./target/x86_64-apple-darwin/"$build_type"/pinion

strip target/aarch64-apple-darwin/"$build_type"/pinion || true
strip target/x86_64-apple-darwin/"$build_type"/pinion || true
lipo -create -output pinion target/aarch64-apple-darwin/"$build_type"/pinion target/x86_64-apple-darwin/"$build_type"/pinion
strip ./pinion || true
chmod u+x ./pinion
if [[ "$RELEASE_COMMIT" == "true" ]]; then
  build_alfred_bundle "$src" "$stage"
elif [[ -n "$PINBOARD_TOKEN" ]]; then
  .github/workflows/run_tests.sh ./pinion "$PINBOARD_TOKEN"
else
  echo "PINBOARD_TOKEN not set; skipping API tests"
fi
