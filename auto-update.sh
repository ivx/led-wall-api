#!/bin/bash


function restart_app {
    pkill led-wall-api
    pkill osc_receiver
    cargo run --bin osc_receiver 0.0.0.0:1234 &
    cargo run --bin led-wall-api &
}

function get_latest_commit {
    git log --shortstat | grep commit | head -n 1 | awk -F ' ' '{print $2}'
}

COMMIT="$(get_latest_commit)"

while true; do
  while [[ $COMMIT = "$(get_latest_commit)" ]]; do
      echo "pulling"
      git pull
      sleep 2
  done
  COMMIT="$(get_latest_commit)"

  echo "restart"
  restart_app
done
