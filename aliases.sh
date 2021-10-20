
### TMUX SESSION MANAGEMENT ALIASES
mkdir -p ~/.tmuxp/

function __saveall {
	NBTOT=$(tmux display-message -p '#{session_windows}')
	for (( w=0; w<$NBTOT; w++ ))
	do  
		__savewin $w
		echo -e "\n\n"
	done
	echo "Done"
}

function __setfocus {
	if [ ! -n "$1" ]; then
		echo "Usage: savelayout <window number>"
		return 1
	fi
	[ "$1" -eq "$1" ] 2>/dev/null || return 1		#Ensure the input is a number
	NWIN=$1
	shift
	SESSION_NAME=$(tmux display-message -p '#S')

	echo "Window $NWIN is now the focused window"
	tmuxp_session_creator edit -n "$SESSION_NAME" -i "$NWIN" -F "$@"
}

function __savewin {
	if [ ! -n "$1" ]; then
		echo "Usage: savelayout <window number>"
		return 1
	fi
	[ "$1" -eq "$1" ] 2>/dev/null || return 1		#Ensure the input is a number
	NWIN=$1
	shift
	PREVWIN=$(tmux display-message -p '#{window_index}')
	tmux select-window -t $NWIN
	SESSION_NAME=$(tmux display-message -p '#S')
	WINDOW_LAYOUT=$(tmux display-message -p '#{window_layout}')
	WINDOW_NAME=$(tmux display-message -p '#{window_name}')
	tmux select-window -t $PREVWIN

	echo "Editing window $WINDOW_NAME"
	tmuxp_session_creator edit -n "$SESSION_NAME" -i "$NWIN" -l "$WINDOW_LAYOUT" -w "$WINDOW_NAME" "$@"
}

alias saveall="test \$TMUX && __saveall"
alias savewin="test \$TMUX && __savewin"
alias setfocus="test \$TMUX && __setfocus"

alias quitses='tmux kill-session; exit 0'
alias listses='ls ~/.tmuxp/ | awk -F "/" "{print $NF}" | cut -d "." -f 1'
alias tmuxload="$(which tmuxp) load"

function _tmuxload_autocomplete {
    COMPREPLY=($(compgen -W "$(listses)" "${COMP_WORDS[1]}"))
}

