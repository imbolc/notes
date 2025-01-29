#!/usr/bin/env sh

file=/tmp/sh-redirect.txt

# rewrite a file
cat >$file <<EOF
1
2
EOF

# append a single line
echo '3' >>$file

# append multiple lines
cat >>$file <<EOF
4
5
EOF

# === On behalf of the root
root_file=/tmp/sh-redirect-root.txt

# rewrite
sudo tee $root_file >/dev/null <<EOF
1
2
EOF

# append (notice -a setting)
echo '3' | sudo tee -a $root_file >/dev/null

# === On behalf of any user
user_file=/tmp/sh-redirect-user.txt
sudo -u nobody tee $user_file >/dev/null <<EOF
1
2
EOF
echo '3' | sudo -u nobody tee -a $user_file >/dev/null

ls -l $file
cat $file

ls -l $root_file
cat $root_file

ls -l $user_file
cat $user_file
