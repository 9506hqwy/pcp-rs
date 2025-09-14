#!/bin/bash
set -euo pipefail

# install libclang-dev and pcp related packages
sudo apt-get update -y
sudo apt-get install -y libclang-dev libpcp3-dev libpcp-pmda3-dev pcp
