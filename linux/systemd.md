Systemd
=======


Timers
------
- enable and start a timer: `sudo systemctl enable --now foo.timer`
- stop and disable a timer: `sudo systemctl disable --now foo.timer`
- list of all started timers: `systemctl list-timers`

For each `.timer` file, a matching `.service` file exists (e.g. `foo.timer` and `foo.service`).
The `.timer` file activates and controls the `.service` file.
The `.service` does not require an `[Install]` section as it is the timer units that are enabled.

### An timer example

Create a service: `/etc/systemd/system/hello.service`

```systemd
[Unit]
Description=Greeting

[Service]
Type=oneshot
User=imbolc
ExecStart=/usr/bin/timeout 1s echo Hey :)
```

And the corresponding timer: `/etc/systemd/system/hello.timer`

```systemd
[Unit]
Description=Greets every 5 minutes

[Timer]
OnCalendar=*:0/5

[Install]
WantedBy=timers.target
```


Enable and start the timer:
```bash
sudo systemctl daemon-reload
sudo systemctl enable --now hello.timer
```

Now you should see a new greeting in the journal each minute:
```sh
journalctl -f -u hello
```
