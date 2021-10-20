# Tmux Session Creator
This project aims to ease the creation of tmuxp JSON file in order to create loadable tmux sessions.

Personnal tool, use it at your own risk

## Setup
- Get [tmuxp](https://github.com/tmux-python/tmuxp)
- Run the script `install.sh`
- Enjoy

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

## Save the modifications on a window
Inside the tmux session, call the `savewin <window number>` command.
This will not affect the commands saved inside the window, only the layout and the window name. To modify the commands inside a window, please use the `setcmds` command.

If a pane has been created since the last save, will ask for commands to save (TODO)

If a pane has been deleted since the last save, will ask for commands to drop (TODO)

## Manually edit the file
All the sessions files are located in `~/.tmuxp/`, with the session name as a json filename.
