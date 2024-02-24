#!/bin/sh
# Exit if there is any failure.
set -e

RELEASE=${RELEASE:-undefined}

TMP=$(mktemp -d)

wget -O $TMP/act.tar.gz https://github.com/nektos/act/releases/$RELEASE/download/act_Linux_x86_64.tar.gz
tar -xzf $TMP/act.tar.gz -C $TMP
mv $TMP/act /usr/local/bin/act
rm -r $TMP
