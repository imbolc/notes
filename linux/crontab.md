crontab
=======

```crontab
MAILTO=""
PATH=/bin:/usr/bin:/usr/local/bin

# run a command with a restricted timeout
15 4 * * *  cd ~/foo && timeout 10m ./manage.py certbot renew

@reboot     /run-something-on-reboot
```
