# Tmux Session Creator
This project aims to ease the creation of tmuxp JSON file in order to create loadable tmux sessions.

Personnal tool, use it at your own risk

## Setup
- Get [tmuxp](https://github.com/tmux-python/tmuxp)
- Build this tool with `cargo build --release`
- Copy the binary `cp ./target/release/tmuxp_session_creator ~/.local/bin/`
- Add some aliases and wrappers to your `~/.bashrc`:
``` bash
mkdir -p ~/.tmuxp/
alias quit='tmux kill-session; exit 0'
alias listses='ls ~/.tmuxp/ | awk -F "/" "{print $NF}" | cut -d "." -f 1'
alias tmuxload="$(which tmuxp) load"
alias savelayout='test $TMUX && tmuxp_session_creator edit -n '"\"$(tmux display-message -p '#S')\" -w \"$(tmux display-message -p '#I')\" -l \"$(tmux display-message -p '#{window_layout}')\"

function _tmuxload_autocomplete {
    COMPREPLY=($(compgen -W "$(listses)" "${COMP_WORDS[1]}"))
}
```

## Create a new session
``` bash
tmuxp_session_creator create -n "session-name" -d /home/me/Projects/name
```
This will create a blank session, with a single window with a single pane executing `bash`

You can describe the windows and panes you want this way:
``` bash
#														Description of window 0									Description of window 1			Focus on window 1
tmuxp_session_creator create -n "name" -d /tmp/proj/ -w "code:./src/:off:0:nvim:cargo-watch -c:clear && bash" -w "shell:.:on:0:clear && bash" -f 1
```

## Load a session
Load any created session using `tmuxpload <session_name>`.
Autocompletion should work

## Save the layout of the current session
Inside the tmux session, call the `savelayout` alias we created.

If a pane has been created since the last save, will ask for commands to save (TODO)

If a pane has been deleted since the last save, will ask for commands to drop (TODO)

## Manually edit the file
All the sessions files are located in `~/.tmuxp/`, with the session name as a json filename.
