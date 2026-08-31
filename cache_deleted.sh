#!/bin/sh

case "$(uname -s)" in
Darwin)
  rm -rf ~/Library/Application\ Support/Gacho
  ;;
Linux)
  data_home="${XDG_DATA_HOME:-$HOME/.local/share}"
  rm -rf "$data_home"/gacho
  ;;
MINGW*|MSYS*|CYGWIN*)
  rm -rf "$APPDATA"/Gacho
  ;;
*)
  echo "unsupported OS: $(uname -s)" >&2
  exit 1
  ;;
esac

exit
