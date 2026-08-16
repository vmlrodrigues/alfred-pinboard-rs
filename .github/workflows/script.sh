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

    # Guard the two ways junk has actually got into a release. Tracking what git
    # tracks is only as good as .gitignore, and renaming the project once turned a
    # name-specific ignore pattern into a hole: a stale 4.4 MB binary and a 3.1 MB
    # bundle nested inside the new one both shipped. Fail loudly instead.
    nested=$(find "$stage" -name '*.alfredworkflow' | wc -l | tr -d ' ')
    if [ "$nested" -ne 0 ]; then
        echo "ERROR: a .alfredworkflow is nested inside the bundle" >&2
        find "$stage" -name '*.alfredworkflow' >&2
        exit 1
    fi
    strays=$(find "$stage" -type f ! -name pinion -exec file {} + | grep -c 'Mach-O' || true)
    if [ "$strays" -ne 0 ]; then
        echo "ERROR: unexpected executable(s) in the bundle besides ./pinion" >&2
        find "$stage" -type f ! -name pinion -exec file {} + | grep 'Mach-O' >&2
        exit 1
    fi

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

# Sign and notarise, so macOS does not refuse to run the binary.
#
# Without this, a downloaded workflow carries a quarantine flag and Gatekeeper
# kills the binary on first use with "Apple could not verify … is free of
# malware", offering "Move to Bin" as the default button. Verified: an ad-hoc
# signed binary is `rejected` by spctl and blocked on execution; the same binary
# signed with Developer ID and notarised is `accepted — source=Notarized
# Developer ID` and runs.
#
# Note there is no `stapler staple` step. Stapling only works on .app/.dmg/.pkg
# bundles — it fails with error 73 on a loose executable like ours. It is not
# needed: Gatekeeper looks the ticket up with Apple online. The only cost is that
# a user who is offline the very first time they use the workflow may still be
# prompted.
#
# Skipped with a warning when the secrets are absent, so pull requests and forks
# still build.
if [[ -n "${MACOS_SIGN_IDENTITY:-}" ]]; then
    echo "==> Signing with $MACOS_SIGN_IDENTITY"
    codesign --force --sign "$MACOS_SIGN_IDENTITY" \
             --options runtime --timestamp ./pinion
    codesign --verify --strict --verbose=2 ./pinion

    if [[ -n "${NOTARY_KEY_PATH:-}" ]]; then
        echo "==> Notarising"
        rm -f ./pinion-notarise.zip
        ditto -c -k --keepParent ./pinion ./pinion-notarise.zip
        xcrun notarytool submit ./pinion-notarise.zip \
            --key "$NOTARY_KEY_PATH" \
            --key-id "$NOTARY_KEY_ID" \
            --issuer "$NOTARY_ISSUER" \
            --wait --timeout 30m
        rm -f ./pinion-notarise.zip
        # Prove the ticket is live before we ship it, rather than trusting the
        # submission result alone.
        spctl -a -vv -t open --context context:primary-signature ./pinion
    else
        echo "WARNING: signed but NOT notarised — no notary credentials" >&2
    fi
else
    echo "WARNING: no signing identity; shipping an unsigned binary." >&2
    echo "         Users will hit Gatekeeper on first run." >&2
fi
if [[ "$RELEASE_COMMIT" == "true" ]]; then
  build_alfred_bundle "$src" "$stage"
elif [[ -n "$PINBOARD_TOKEN" ]]; then
  .github/workflows/run_tests.sh ./pinion "$PINBOARD_TOKEN"
else
  echo "PINBOARD_TOKEN not set; skipping API tests"
fi
