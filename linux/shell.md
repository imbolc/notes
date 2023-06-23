# Linux shell

- status of the last command (0 - for success): `echo $?`

Redirecting `sudo` output to file:
```sh
ls /root | sudo tee ls.txt > /dev/null
```

And for multi-line text:
```sh
sudo tee file.txt > /dev/null << EOF
foo
bar
baz
EOF
```

Adding multiple lines instead of replacing the file content:
```sh
sudo tee -a file.txt > /dev/null << EOF
foo
bar
baz
EOF
```

Bash options:
```bash
set -o nounset  # error when referencing undefined variable
set -o errexit  # exit when command fails
```
