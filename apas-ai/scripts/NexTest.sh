#!/bin/bash

cargo nextest run 2>&1 | sed 's/\x1b\[[0-9;]*m//g; s/\x1b\[[0-9]*[ABCDEFGHJKST]//g'
