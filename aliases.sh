
### TMUX SESSION MANAGEMENT ALIASES
mkdir -p ~/.tmuxp/

function __savewin {
	[ "$1" -eq "$1" ] 2>/dev/null || return 1		#Ensure the input is a number
	NWIN=$1
	PREVWIN=$(tmux display-message -p '#{window_index}')
	tmux select-window -t $NWIN
	SESSION_NAME=$(tmux display-message -p '#S')
	WINDOW_LAYOUT=$(tmux display-message -p '#{window_layout}')
	WINDOW_NAME=$(tmux display-message -p '#{window_name}')
	tmuxp_session_creator edit -n "$SESSION_NAME" -i "$NWIN" -l "$WINDOW_LAYOUT" -w "$WINDOW_NAME"
	tmux select-window -t $PREVWIN
}

alias savewin="test \$TMUX && [ -n \"\$1\" ] || echo \"Usage: savelayout <window number>\" && __savewin"

function __setcmds {
	[ "$1" -eq "$1" ] 2>/dev/null || return 1		#Ensure the input is a number
	NWIN=$1
	tmuxp_session_creator setcmds -i "$NWIN"
}

alias setcmds="test \$TMUX && [ -n \"\$1\" ] || echo \"Usage: setcmds <window number>\" && __setcmds"

alias quitses='tmux kill-session; exit 0'
alias listses='ls ~/.tmuxp/ | awk -F "/" "{print $NF}" | cut -d "." -f 1'
alias tmuxload="$(which tmuxp) load"

function _tmuxload_autocomplete {
    COMPREPLY=($(compgen -W "$(listses)" "${COMP_WORDS[1]}"))
}

