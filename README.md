# Tmux Session Creator
This project aims to ease the creation of tmuxp JSON file in order to create loadable tmux sessions.
## Setup
- Get [tmuxp](https://github.com/tmux-python/tmuxp)
- Add some aliases and wrappers to your `~/.bashrc`:
``` bash
mkdir -p ~/.tmuxp/
alias quit='tmux kill-session; exit 0'
alias listses='ls ~/.tmuxp/ | awk -F "/" "{print $NF}" | cut -d "." -f 1'
alias tmuxload="$(which tmuxp) load"

function _tmuxload_autocomplete {
    COMPREPLY=($(compgen -W "$(listses)" "${COMP_WORDS[1]}"))
}
```
- Use the tool to create new sessions in the `~/.tmuxp/` directory:
``` bash
tmuxp_session_creator create -n "session-name" -d /home/me/Projects/name ...
```
- Then load the created session using `tmuxpload <session_name>`
Enjoy

# WORK IN PROGRESS, not working version yet
