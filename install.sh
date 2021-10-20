#!/bin/bash

cargo build --release
cp ./target/release/tmuxp_session_creator $HOME/.local/bin/tmuxp_session_creator
grep "__savewin" $HOME/.bashrc 1>/dev/null || cat aliases.sh >> $HOME/.bashrc
