#!/bin/bash

cargo build --release
cp ./target/release/tmuxp_session_creator $HOME/.local/bin/tmuxp_session_creator
cp aliases.sh $HOME/.tmuxp_session_creator_aliases.sh

grep "source \$HOME/.tmuxp_session_creator_aliases.sh" $HOME/.bashrc 1>/dev/null || echo "source \$HOME/.tmuxp_session_creator_aliases.sh" >> $HOME/.bashrc
