WRONG_EMAIL="david.volkers@gwdg.de"
NEW_NAME="LeSnake04"
NEW_EMAIL="dev.lesnake@poseto.com"

if [ "$GIT_COMMITTER_EMAIL" = "$WRONG_EMAIL" ]; then
	export GIT_COMMITTER_NAME="$NEW_NAME"
	export GIT_COMMITTER_EMAIL="$NEW_EMAIL"
fi
if [ "$GIT_AUTHOR_EMAIL" = "$WRONG_EMAIL" ]; then
	export GIT_AUTHOR_NAME="$NEW_NAME"
	export GIT_AUTHOR_EMAIL="$NEW_EMAIL"
fi
