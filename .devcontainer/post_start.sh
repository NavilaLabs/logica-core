#!/bin/bash

#############
# rust-base #
#############

git config user.signingkey ~/.ssh/id_ed25519.pub --global
git remote add template git@github.com:NavilaLabs/rust-base.git 2>/dev/null || true

chmod +x ./git/merge_template.sh
