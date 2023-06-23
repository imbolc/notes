# Users and permissions

- show user's groups: `groups user`
- add user to a group: `usermod -a -G group user` or `adduser user group`

## Sudo without password

- add user to `sudo` group: `usermod -a -G sudo user`
- change `/etc/sudoers` with `visudo`: `%sudo   ALL=(ALL) NOPASSWD: ALL`


## Add an user without prompt

    adduser --gecos "" username

## Remove an user with it's home folder

    sudo userdel -r username
