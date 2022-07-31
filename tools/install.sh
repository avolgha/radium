#!/bin/bash

# this script helps the user to install/ build the executable by there self.

error() {
	echo -e "\e[1;31m[error]\e[0m $*"
}

info() {
	echo -e "\e[1;32m[info]\e[0m $*"
}

dist_path="$OUTPUT_PATH"
bin_path="~/.local/bin/"
comp_path="~/.completions/"

if [ "$dist_path" == "" ]; then
	dist_path="/opt"
fi

cd "$dist_path"

info "cloning repository"
sudo git clone https://github.com/avolgha/radium.git &1>/dev/null

info "building the binary"
cd radium
sudo make radium &1>/dev/null

info "moving binary to '$bin_path'"
if [ ! -d "$bin_path" ]; then
	mkdir -p "$bin_path"
fi

sudo mv target/release/radium "$bin_path"

info "adding shell completion"
if [ ! -d "$comp_path" ]; then
	mkdir -p "$comp_path"
fi

sudo cp completions/completion.bash "${comp_path}radium.bash"

echo "source ${comp_path}radium.bash" >> ~/.bashrc

clear
info "radium was installed successfully."
