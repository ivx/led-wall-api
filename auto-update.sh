#!/bin/bash


function restart_app {
    pkill led-wall-api
    pkill osc_receiver
    cargo run --bin osc_receiver 0.0.0.0:1234 > receiver.log &
    cargo run --bin led-wall-api > led-wall-api.log &
}

function get_latest_commit {
    git log --shortstat | grep commit | head -n 1 | awk -F ' ' '{print $2}'
}

COMMIT="$(get_latest_commit)"

while true; do
  while [[ $COMMIT = "$(get_latest_commit)" ]]; do
      echo "."
      git pull $1 > /dev/null
      sleep 2 $1 > /dev/null
  done
  COMMIT="$(get_latest_commit)"

  echo "restart"
  restart_app
done
