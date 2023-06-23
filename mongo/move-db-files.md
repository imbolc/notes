# Move mongo database to a new location

- `sudo /etc/init.d/mongodb stop`
- open `/etc/mongodb.conf`
- change `dbPath: /new/database/location`
- move database folder to the new location
- `sudo /etc/init.d/mongod start`
