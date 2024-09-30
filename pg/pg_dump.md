# Postgres dump

## SQL Dump

- it creates a consistent dump of database at the running time
- this dump procedure doesn't block other database operations,
  except of those that need exclusive lock (e.g. `ALTER TABLE`)
- it usually can be restored into newer postgres version
- it's the only method to transfer a database to another
  hardware architecture (e.g. 32bit to 64bit)

```sh
DB=dbname bash -c 'pg_dump -O $DB | xz -T0 > $DB.sql.xz'
DB=dbname bash -c 'unxz -c $DB.sql.xz | psql $DB'
```

```sh
pg_dump --no-owner --no-acl dbname | gzip > "dump-$(date -u +"%Y-%m-%dT%H:%M:%SZ").sql.gz"
gunzip -c dump.sql.gz | psql dbname
```

## Continuous Archiving and Point-in-Time Recovery (PITR)

- `pg_dump` **can NOT** be used as a starting point (base backup)
  for this type of recovery while inconsistent file system backup can
- archiving of WAL files should be started before the base backup
- `archive_command` isn't a suitable way to do backup as it runs only
  after a segment file is completed or due to `active_timout`. So in case of
  a crush the incomplete segment, containing last transactions, can be lost.
  So we'll be using `pg_receivewal` to stream WAL files.

Set `postgresql.conf` parameters:

    # The WAL level must be `replica` or `logical`
    wal_level = replica

Enable replication in `pg_hba.conf`:

    local   replication     postgres                                peer

Restart the db.

### Receivewal

To avoid losing of last incomplete WAL segment using `archive_command`,
we'll `pg_receivewal` to stream even unfinished WAL segments.
When we start it first time with a particular slot it will start fetching
from the current WAL segment.

The purpose of _replication slot_ is to track how much WAL files is necessary to
keep for a particular standby synchronization. Let's create one:
`sudo -u postgres pg_receivewal -S slot1 --create-slot`

Now let's try to receive logs:
`sudo -u postgres pg_receivewal -S slot1 -Z9 -v -D /backup/wal`

If everything works let's make it permanent by e.g. using supervisor:

```
sudo tee /etc/supervisor/conf.d/receivewal.conf > /dev/null << EOF
[program:receivewal]
user=postgres
directory=/backup/wal
command=pg_receivewal -S slot1 -Z9 -v -D /backup/wal
autorestart=true
stdout_logfile=NONE
stderr_logfile=NONE
EOF
```

### Receivewal monitoring

We can look at active streaming by `select * from pg_stat_replication;`
It shows **only connected**

- `state` - should be `streaming`
- `write_lag` - shouldn't be too big up to 10 seconds or something

## Base backup

After activating Continuous Archiving you should do a base backup as a starting
point for recovery. It makes backup of the entire database cluster.

```sh
sudo -u postgres pg_basebackup -D - -Ft -X fetch | xz -9 | \
sudo -u postgres tee "/backup/base/$(date +"%Y-%m-%dT%H-%M-%S").tar.xz" > /dev/null
``

You can do it on a daily basis with crontab job:
`0 5 * * * sudo -u postgres ...`


Recovery
--------
Official docs: [chapter 25.3.4](https://www.postgresql.org/docs/9.6/continuous-archiving.html)

1. Make sure database server is stopped
2. Backup the entire db cluster folder if it's possible,
   or at least `pg_wal/` (`pg_xlog`) subdirectory, as it might contain not archived
   logs
3. Remove all existing files and subdirectories under the cluster data directory
4. Restore files from base backup, make sure they're owned by `postgres` user
5. Remove any files present in `pg_wal/` (`pg_xlog`) folder (which came with base
   backup)
6. Copy WAL segments saved in step 2 to `$PGDATA/pg_wal`
7. If you have a `.partial` WAL segment in `pg_receivewal` backup and you don't
   have this segment completed (saved in step 2), remove `.partial` extension:

        PGDATA=/data/dbs/postgresql/11/main find ./ -name "*.partial" -exec sh -c ' \
            NAME=`basename "$0" .gz.partial`; \
            DEST=$PGDATA/pg_wal/$NAME; \
            test ! -f $DEST && \
            zcat $0 > $NAME && \
            truncate -s16M $NAME && \
            mv $NAME $DEST \
        ' '{}' \;

8. Create a recovery file `$PGDATA/recovery.conf`

        # a required parameter
        # %f - is a WAL segment file name
        # %p - path where the file would be copied to
        restore_command = 'gzip -d < /backup/wal/%f.gz > %p'

        # for PITR or if normal recovery breaks at some point
        # recovery_target_time = '2004-07-14 22:39:00 UTC'

9. Start the server. To see any possible errors, you can start it without
   demonization by something like:
   `/usr/lib/postgresql/11/bin/pg_ctl start -D /etc/postgresql/11/main`
10. After a successful recovery the `recovery.conf` should be renamed to `recovery.done`
```
