# Move postgres database to a new location

## Symlink approach

Simple move the postgres data folder to a new location:
```bash
$new_location=/data
sudo systemctl stop postgresql
sudo mv /var/lib/postgresql $new_loaction/
sudo ln -s $new_loaction/postgresql /var/lib/
sudo systemctl start postgresql
```

## Changing config file

Or you can change the config file after copying the data folder instead
of symlinking:

- open `/etc/postgresql/<pg-version>/main/postgresql.conf`
- change `data_directory = '/new/database/location'`
