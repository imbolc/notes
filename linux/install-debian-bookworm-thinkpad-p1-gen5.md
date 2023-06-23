# Install Debian 12 Bookworm on ThinkPad P1 Gen 5

## From root
```sh
apt update && apt upgrade -y
```
### Vimification
```sh
apt install curl
bash <(curl -sL https://raw.github.com/imbolc/server-setup/master/partials/vimification.sh)
bash
```
### Disable sudo password for the main user
```sh
read -p "Enter a username for sudo user: " -i user -e sudo_user
echo "$sudo_user ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/$sudo_user
```

### Hybrid graphics
```sh
apt install -y bumblebee-nvidia primus mesa-utils xserver-xorg-input-mouse
ln -s /usr/share/X11/xorg.conf.d /etc/bumblebee/
```
Uncomment `BusID` in `/etc/bumblebee/xorg.conf.nvidia`

### Restart the system
```sh
systemctl reboot
```

## From a normal user

### Test hybrid graphics

Integrated card
```sh
glxinfo | grep "OpenGL renderer"
```
Nvidia card
```sh
optirun glxinfo | grep "OpenGL renderer"
```

### Libs
```sh
sudo apt install \
  libbz2-dev \
  libcairo2 \
  libffi-dev \
  liblzma-dev \
  libncursesw5-dev \
  libpango1.0-0 \
  libreadline-dev \
  libsqlite3-dev  \
  libssl-dev \
  libsystemd-dev \
  libxml2-dev \
  libxmlsec1-dev \
  tk-dev \
  zlib1g-dev \
```

### CLI tools
```sh
sudo apt install \
  build-essential \
  curl \
  git \
  highlight \
  htop \
  llvm \
  make \
  mc \
  mosh \
  ncdu \
  nodejs \
  npm \
  postgresql \
  python3-pip \
  python3-venv \
  ranger \
  redis \
  rsync \
  tmux \
  tree \
  wget \
  wrk \
  xz-utils \
```

Enable password-less access to postgres
```sh
sudo su postgres -c "cd /; createuser -s $USER"
```

### GUI apps
```sh
sudo apt install \
  blueman \
  flameshot \
  gitk \
  kazam \
  keepassxc \
  openvpn \
  qbittorrent \
  terminator \
  vlc \
  xclip \
```
Set `flameshot gui` on `PrtSrc`

### Vim
```sh
sudo apt remove -y vim 
sudo curl -L https://github.com/neovim/neovim/releases/latest/download/nvim.appimage -o /usr/local/bin/vim
sudo chmod +x /usr/local/bin/vim

git clone git@github.com:imbolc/nvim.git ~/.config/
git clone git@github.com:imbolc/notes.git ~/Documents/
# throws errors, could make sense to run multiple times
vim --headless -c 'autocmd User PackerComplete quitall' -c 'PackerSync'
```

### Rust
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add x86_64-unknown-linux-musl
rustup component add rustfmt clippy rust-src rust-analyzer

cargo install --locked \
  bottom \
  cargo-expand \
  cargo-generate \
  cargo-limit \
  cargo-readme \
  cargo-sync-readme \
  cargo-watch \
  comrak \
  fd-find \
  git-delta \
  mask \
  ripgrep \
  rust-script \
  rusty-hook \
  skim \
  sqlx-cli \
  stylua \
  taplo-cli \
  tealdeer \
  typos-cli \
  xplr \
```
### Limit CPU performance
```sh
sudo apt install powercap-utils
sudo tee -a /etc/systemd/system/cpu-powercap.service > /dev/null << EOF
[Unit]
Description=Limit CPU performance

[Service]
ExecStart=/usr/bin/powercap-set -p intel-rapl -z 0 -c 1 -l 30000000
Type=oneshot
RemainAfterExit=yes
User=root

[Install]
WantedBy=multi-user.target
EOF
sudo systemctl enable cpu-powercap
sudo systemctl start cpu-powercap
sudo systemctl status cpu-powercap
```
