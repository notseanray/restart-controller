#!/bin/bash
cargo build --release
mkdir -p ~/.config/systemd/user/
cp restart-controller.service ~/.config/systemd/user/
sudo cp target/build/restart-controller /usr/local/bin/
sudo mkdir /etc/restart-controller
sudo touch /etc/restart-controller/config.json
