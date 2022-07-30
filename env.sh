#!/bin/bash

# this script adds the development executable builds to the
# bash $PATH variable.

export PATH="$PATH:target/debug"

alias sradium="sudo target/debug/radium"
