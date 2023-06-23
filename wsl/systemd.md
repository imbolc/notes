# Enabling systemd on wsl2

1. Install [arkane-systems/genie](https://github.com/arkane-systems/genie)
```bash
sudo su

wget -O /etc/apt/trusted.gpg.d/wsl-transdebian.gpg https://arkane-systems.github.io/wsl-transdebian/apt/wsl-transdebian.gpg

chmod a+r /etc/apt/trusted.gpg.d/wsl-transdebian.gpg

cat << EOF > /etc/apt/sources.list.d/wsl-transdebian.list
deb https://arkane-systems.github.io/wsl-transdebian/apt/ $(lsb_release -cs) main
deb-src https://arkane-systems.github.io/wsl-transdebian/apt/ $(lsb_release -cs) main
EOF

apt update
sudo apt install -y systemd-genie
```

2. Add to the top of `~/.bashrc`
```bash
if [[ ! -v INSIDE_GENIE ]]; then
    exec /usr/bin/genie -c bash
fi
```
