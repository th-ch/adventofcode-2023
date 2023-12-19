#!/bin/bash

set -ev

# source $HOME/.cargo/env
export PATH=$PATH:~/.cargo/bin:$GOROOT/bin

./aoc run --all

# Upload the content of leaderboard to the web.
netlify deploy --prod --site cs-advent-of-code-2023 -d leaderboard --auth $NETLIFY_AUTH

