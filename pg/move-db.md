# Move postgres database to a new location

- `sudo systemctl stop postgresql`
- open `/etc/postgresql/<pg-version>/main/postgresql.conf`
- change `data_directory = '/new/database/location'`
- move database folder to the new location
- `sudo /etc/init.d/postgresql start`
