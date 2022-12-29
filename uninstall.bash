#!/usr/bin/env bash

print "\033[1;36m[I]: Initiating uninstallation script...\033[0m\n";
read -s "\033[1;36m[?]: Continue with uninstallation?[y/N]: " CONFIRM

if $CONFIRM -ne 'y' then
    print "\033[1;31m[!]: Aborting \033[0m\n";
    exit 0
fi

## Fetching the env file from config for information on project directory:
source $HOME/netpulse/env

## Removing the netpulse config file:
print "\033[1;36m[I]: Removing the netpulse config file: $HOME/.netpulse\033[0m\n";
rm -rf $HOME/.netpulse

read -s "\033[1;36m[?]: Do you wish to remove the project directory and source code?[y/N]: " $SRCCONFIRM
if $SRCCONFIRM -eq 'y' then
    print "\033[1;36m[I]: Removing netpulse source code and project directory: $NPULSE_ROOT\033[0m\n";
    rm -rf $NPULSE_ROOT
    print "\033[1;32m[+]: Done!\033[0m\n"; 

else
    print "\033[1;36m[I]: Preserving $NPULSE_ENV\033[0m\n";
fi

print "\033[1;36m[I]: netpulse has successfully uninstalled.\033[0m\n";
