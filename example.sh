#!/bin/bash

wsm="./strings2set2sqlite.wasm"

hdir="$PWD"
gdir="/guest.d"
bname="example.sqlite.db3"
gpath="${gdir}/${bname}"
hpath="${hdir}/${bname}"

run_wasi(){
  cat /dev/stdin |
    wasmtime \
      run \
      --env ENV_SQLITE_DB_FILE_PATH="${gpath}" \
      --dir "${hdir}::${gdir}" \
      "${wsm}"
}

input1(){
  echo wrld
  echo helo
  echo world
  echo hello
  echo helo
  echo helo
  echo helo
}

echo converting the strings to a set...
input1 | run_wasi
echo

echo printing the set
sqlite3 \
  -markdown \
  "${hpath}" \
  "
    SELECT key FROM strings
    ORDER BY key
  "
