# Systemd


## Timers

Timers added by creating two files which the same name, but different extensions:

- `foo.service` - the one-shot service that runs the intended command
- `foo.timer` - that's specifies the timer behaviour

### Useful commands

```bash
sudo systemctl enable --now foo.timer    # Enable and start a timer
sudo systemctl disable --now foo.timer   # Stop and disable a timer
systemctl list-timers                    # List all started timers
```

### A timer example

Let's create a one-shot greeting service that prints a string into the system journal:

```bash
sudo tee /etc/systemd/system/hello.service > /dev/null << EOF
[Service]
Type=oneshot
ExecStart=echo "Hey :)"
EOF
```

We can run it and check the journal for the output:

```bash
sudo systemctl start hello
sudo journalctl -u hello
```

Now let's create the corresponding timer to automatically run the service every second.
Timer bound to the service by the filename `hello.timer` starts `hello.service`.

```bash
sudo tee /etc/systemd/system/hello.timer > /dev/null << EOF
[Timer]
# Run the service 1 second after boot
OnBootSec=1s

# Run the service again in 5 seconds after it deactivates
OnUnitActiveSec=5s

[Install]
# Start the timer automatically on boot
WantedBy=timers.target
EOF
```

Now we can enable the timer:

```bash
sudo systemctl daemon-reload  # Reload systemd configs after modification
sudo systemctl enable --now hello.timer
```

And watch the system journal for the greeting messages appearing every 5 seconds:

```sh
sudo journalctl -f -u hello
```
