#!/usr/bin/env bash


: '
    Author: Aliasgar Khimani (NovusEdge)
    Project: github.com/NovusEdge/netpulse
    Copyright: GNU General Public License v3.0
    See the LICENSE file for more info.
    All Rights Reserved
'

VERSION="v1.0.0"

printf "\033[1;36m=================LICENSE NOTICE=================\n";
echo "\
    Author: Aliasgar Khimani (NovusEdge)
    Project: github.com/NovusEdge/netpulse
    Version: $VERSION
    Copyright: GNU General Public License v3.0
    See the LICENSE file for more info.
    All Rights Reserved
=================================================
"; printf "\033[0m"


PROJECT_DIR=$(dirname $(realpath $0))

if ! type "cargo" > /dev/null ; then
    printf "\033[1;31m[E]: Cargo not detected!\n";
    printf "     Please install it from: https://www.rust-lang.org/tools/install \033[0m\n\n";
    exit 1;
else
    printf "\033[1;32m[+]: Found cargo installation at: \n\t\033[35m$(which cargo)\033[0m\n\n";
    RUST=$(which rustc)
    CARGO=$(which cargo)
    RUSTUP=$(which rustup)
fi

oldcwd=$(pwd)
printf "\033[1;36m[I]:Fetching project dependencies...\033[0m\n";

cd $PROJECT_DIR

$CARGO update
$CARGO build

cd $oldcwd
printf "\033[1;32m[+]: Done!\033[0m\n\n";


## Making ~/.netpulse if it does not exist.
if [ ! -d $HOME/.netpulse ]; then
    printf "\033[1;36m[I]: Making .netpulse in $HOME for storing config files and logs...\033[0m\n";
    mkdir $HOME/.netpulse
    touch $HOME/.netpulse/env
    printf "\033[1;32m[+]: Done!\033[0m\n\n";
fi

## Setting up ~/.netpulse/env
printf "\033[1;36m[I]: Setting up environment file...\033[0m\n";

echo NPULSE_ROOT="$PROJECT_DIR"             > $HOME/.netpulse/env
echo NUPLSE_VER="$VERSION"                 >> $HOME/.netpulse/env
echo NPULSE_ENV="$HOME/.netpulse/env"      >> $HOME/.netpulse/env
echo NPULSE_BIN="$PROJECT_DIR/bin"         >> $HOME/.netpulse/env

printf "\033[1;32m[+]: Done!\033[0m\n\n";


