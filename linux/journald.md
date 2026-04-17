# Journald

## Wrap lines by default

```sh
grep -qxF 'export SYSTEMD_LESS=FRXMK' ~/.bashrc || printf '\nexport SYSTEMD_LESS=FRXMK\n' >>~/.bashrc
```
