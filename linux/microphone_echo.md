Microphone echo cancellation
============================

Add to `/etc/pulse/default.pa`:

```
load-module module-echo-cancel use_master_format=1 aec_method=webrtc source_name=noecho
set-default-source noecho
```

Restart pulseaudio:

```sh
pulseaudio -k
pulseaudio --start
```
