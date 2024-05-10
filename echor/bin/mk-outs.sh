#!/usr/bin/env bash

OUTDIR="tests/expected"
if [ ! -d "$OUTDIR" ]; then
  mkdir -p "$OUTDIR"
fi

echo "Hello there" > $OUTDIR/hello1.txt
echo "Hello" "there" > $OUTDIR/hello2.txt
echo -n "Hello there" > $OUTDIR/hello3.txt
echo -n "Hello" "there" > $OUTDIR/hello4.txt
