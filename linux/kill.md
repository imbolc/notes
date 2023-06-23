Kill
====

- check who is listening to port: `netstat -tulpn | grep :3000`
- kill by port: `kill -9 $(lsof -t -i:3000)`
- kill by name: `killall <name>`

