#!/bin/bash

LOG_FILE="$PWD/output.log"

touch $LOG_FILE

check_if_installed () {
    if ! [ -x "$(command -v $1)" ]; then
        echo "$1 not installed"
        echo "install with: $2"
        exit 1
    fi
}

check_if_installed 'cargo' 'curl https://sh.rustup.rs -sSf | sh'
check_if_installed 'cargo-watch' 'cargo install cargo-watch'
check_if_installed 'nginx' 'sudo apt install nginx'

prepend () { 
    while read line; 
        do echo -e "${1}\t${line}"; 
    done;
}

# reverse proxy with CORS for debugging
run_nginx () {
    echo "Starting nginx"
    nginx -c $PWD/nginx.conf -p $PWD/
}

run_svelte () {
    echo "Starting svelte"
    $(cd client && npm run dev | prepend "svelte:" | tee -a $LOG_FILE) 
}
run_cargo () {
    echo "Starting cargo"
    cargo watch --ignore client --ignore db.sqlite3 -x run | prepend "cargo:" | tee -a $LOG_FILE
}

# trap ctrl-c and kill all subprocesses
trap 'kill $(jobs -p)' SIGINT SIGTERM

# run the 3 functions
run_nginx &
run_svelte &
run_cargo &

wait