# Upgrade Postgres version from 9.6 to 11

Based on this [stackoverflow thread][stackoverflow]

1. Make a dump: `pg_dumpall | xz -9 > ./dumpall.sql.xz`
1. Install a [new version of db][install-db]
1. `pg_lsclusters` should show an old and a new cluster running together
1. Drop the new cluster: `sudo pg_dropcluster 11 main --stop`
1. Stop database server: `sudo systemctl stop postgresql`
1. Make one more backup just in case: `sudo tar cfJ ./backup.tar.xz /var/lib/postgresql/9.6/main`
1. Upgrade the old cluster: `sudo pg_upgradecluster -m upgrade 9.6 main`
1. Start the server: `sudo systemctl start postgresql`
1. `pg_lsclusters` should show the old cluster is down and the new one is online
1. If everything works ok, remove the old cluster: `sudo pg_dropcluster 9.6 main --stop`


[stackoverflow]: https://stackoverflow.com/questions/46687645/upgrade-postgresql-from-9-6-to-10-0-on-ubuntu-16-10
[install-db]: https://www.postgresql.org/download/linux/debian/
