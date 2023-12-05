#!/bin/bash

set -ev

# source $HOME/.cargo/env
export PATH=$PATH:~/.cargo/bin:$GOROOT/bin

./aoc run --all

# Upload the content of leaderboard to the web.