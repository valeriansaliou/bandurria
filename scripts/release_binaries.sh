#!/bin/bash

##
#  Bandurria
#
#  Lightweight comment system for static websites
#  Copyright: 2025, Valerian Saliou <valerian@valeriansaliou.name>
#  License: Mozilla Public License v2.0 (MPL v2.0)
##

# Read arguments
while [ "$1" != "" ]; do
    argument_key=`echo $1 | awk -F= '{print $1}'`
    argument_value=`echo $1 | awk -F= '{print $2}'`

    case $argument_key in
        -v | --version)
            # Notice: strip any leading 'v' to the version number
            BANDURRIA_VERSION="${argument_value/v}"
            ;;
        *)
            echo "Unknown argument received: '$argument_key'"
            exit 1
            ;;
    esac

    shift
done

# Ensure release version is provided
if [ -z "$BANDURRIA_VERSION" ]; then
  echo "No Bandurria release version was provided, please provide it using '--version'"

  exit 1
fi

# Define release pipeline
function release_for_architecture {
    final_tar="v$BANDURRIA_VERSION-$1.tar.gz"

    rm -rf ./bandurria/ ./target/ && \
        cross build --target "$2" --release && \
        mkdir ./bandurria && \
        cp -p "target/$2/release/bandurria" ./bandurria/ && \
        cp -r ./config.cfg ./res bandurria/ && \
        tar --owner=0 --group=0 -czvf "$final_tar" ./bandurria && \
        rm -r ./bandurria/
    release_result=$?

    if [ $release_result -eq 0 ]; then
        echo "Result: Packed architecture: $1 to file: $final_tar"
    fi

    return $release_result
}

# Run release tasks
ABSPATH=$(cd "$(dirname "$0")"; pwd)
BASE_DIR="$ABSPATH/../"

rc=0

pushd "$BASE_DIR" > /dev/null
    echo "Executing release steps for Bandurria v$BANDURRIA_VERSION..."

    release_for_architecture "x86_64" "x86_64-unknown-linux-musl" && \
        release_for_architecture "aarch64" "aarch64-unknown-linux-musl"
    rc=$?

    if [ $rc -eq 0 ]; then
        echo "Success: Done executing release steps for Bandurria v$BANDURRIA_VERSION"
    else
        echo "Error: Failed executing release steps for Bandurria v$BANDURRIA_VERSION"
    fi
popd > /dev/null

exit $rc
