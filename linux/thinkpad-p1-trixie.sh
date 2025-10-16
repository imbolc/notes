# Install Debian 13 Trixie on ThinkPad P1
#
# The script is meant for copy-pasting and running part by part

git clone /media/imbolc/private-backup/gems
rm -rf ~/.ssh
rm -rf ~/.gnupg
~/gems/link.sh

## Passwordless sudo
echo "imbolc ALL=(ALL) NOPASSWD: ALL" >/etc/sudoers.d/imbolc
sudo su -c "echo $USER"

## Dotfiles

cd
git clone git@github.com:imbolc/dotfiles.git
mv dotfiles/.git ./
mv dotfiles/.gitignore ./
git checkout .

## Vimification
sudo apt install -y curl
bash <(curl -sL https://raw.github.com/imbolc/server-setup/master/partials/vimification.sh)
sudo su -c 'bash <(curl -sL https://raw.github.com/imbolc/server-setup/master/partials/vimification.sh)'
bash

## Rust
sudo apt install -y curl mold
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

## Neovim

bash
sudo apt install -y git nodejs npm pipx xclip
git clone git@github.com:imbolc/nvim.git ~/.config/nvim
git clone git@github.com:imbolc/notes.git ~/Documents/notes
upgrade-vim
cd ~/.config/nvim/
./install.sh
vim

## NVM
cd
mkdir .nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash
bash
nvm install 22

## Upgrade everything
upgrade

# Don't lock screen on inactivity
sudo apt purge -y light-locker

# CLI tools
sudo apt install -y \
    aria2 \
    build-essential \
    bash-completion \
    highlight \
    htop \
    llvm \
    make \
    mc \
    mosh \
    ncdu \
    python3-pip \
    python3-venv \
    ranger \
    rsync \
    tmux \
    tree \
    wget \
    wrk \
    xz-utils

# GUI apps
sudo apt install \
    android-file-transfer \
    blueman \
    flameshot \
    gnome-disk-utility \
    keepassxc \
    qbittorrent \
    simplescreenrecorder \
    vlc

# Postgres
sudo apt install -y postgresql
sudo su postgres -c "cd /; createuser -s $USER"

# Autostart
ln -s ~/.local/share/applications/alacritty.desktop ~/.config/autostart/

# Mate hotkeys
gsettings set com.solus-project.brisk-menu hot-key '<Alt>space'                     # Set brisk-menu launcher to Alt+Space
gsettings set org.mate.Marco.keybinding-commands command-screenshot 'flameshot gui' # Make Flameshot a default screenshoter

# Nvidia driver (doesn't work with external display)

sudo tee /etc/apt/sources.list >/dev/null <<EOF
deb http://deb.debian.org/debian/ trixie main contrib non-free non-free-firmware
deb http://security.debian.org/debian-security trixie-security main contrib non-free non-free-firmware
deb http://deb.debian.org/debian/ trixie-updates main contrib non-free non-free-firmware
EOF
sudo apt update
sudo apt install linux-headers-"$(uname -r)"
sudo apt install -y nvidia-driver firmware-misc-nonfree
