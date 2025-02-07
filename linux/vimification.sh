# Vimification

# shellcheck disable=SC2016
vi_bashrc='set -o vi; export VISUAL=vim; export EDITOR="$VISUAL"'
vi_inputrc='set editing-mode vi'
echo "$vi_bashrc" >>~/.bashrc
echo "$vi_inputrc" >>~/.inputrc
echo "$vi_bashrc" | sudo tee -a /root/.bashrc
echo "$vi_inputrc" | sudo tee -a /root/.inputrc

# Other users
echo "$vi_bashrc" | sudo -u myuser tee -a /home/myuser/.bashrc
echo "$vi_inputrc" | sudo -u myuser tee -a /home/myuser/.inputrc
