Open port for a specific IP only
================================

Opens port 27017 for localhost and 1.1.1.1 only.

```
#!/usr/sbin/nft -f

flush ruleset

table inet filter {
        chain input {
                type filter hook input priority 0;
                ip saddr 1.1.1.1 tcp dport 27017 accept
                tcp dport 27017 drop
        }
        chain forward {
                type filter hook forward priority 0;
        }
        chain output {
                type filter hook output priority 0;
        }
}
```

- firewall
- nft
- nftables
