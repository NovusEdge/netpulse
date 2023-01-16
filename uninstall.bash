#!/usr/bin/env bash

printf "\033[1;36m[I]: Initiating uninstallation script...\033[0m\n"
printf "\033[1;36m[?]: Continue with uninstallation?[y/N]: "
read CONFIRM

if ! [[ "$CONFIRM" = "y" || "$CONFIRM" = "Y" ]] then
    printf "\033[1;31m[!]: Aborting \033[0m\n"
    exit 0
fi

## Fetching the env file from config for information on project directory:
source $HOME/.netpulse/env

## Removing the netpulse config file:
printf "\033[1;36m[I]: Removing the netpulse config file: $HOME/.netpulse\033[0m\n"
rm -rf $HOME/.netpulse

printf "\033[1;36m[?]: Do you wish to remove the project directory and source code?[y/N]: "
read SRCCONFIRM
if [[ "$SRCCONFIRM" = "y" || "$SRCCONFIRM" = "Y" ]] then
    printf "\033[1;36m[I]: Removing netpulse source code and project directory: $NPULSE_ROOT\033[0m\n"
    rm -rf $NPULSE_ROOT
    printf "\033[1;32m[+]: Done!\033[0m\n"

else
    printf "\033[1;36m[I]: Preserving $NPULSE_ENV\033[0m\n"
fi

printf "\033[1;36m[I]: netpulse has successfully uninstalled.\033[0m\n"
