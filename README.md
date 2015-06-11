# fasterdns

This is a DNS proxy server that tries to speed up DNS resolution for you. It
works running a DNS server locally. It forwards incoming requests to all the
public DNS servers, and as soon as one of those real DNS servers answers, it
responds with the answer. It ignores the (slow) answers from all the other DNS
servers.

This is probably a jerk thing to do. Sorry, DNS servers!

## Use it

Build it with Cargo:

```
$ cargo build
```

Run it as root (necessary to bind to privileged port 53):

```
$ sudo cargo run
```

Configure you machine's DNS.

OSX:

```
$ sudo networksetup -setdnsservers Wi-Fi 127.0.0.1
```

Enjoy blazing fast DNS lookups. It logs out interactions right now so you can
see which server responds the speediest.


# The Future

- Make the logging optional via flag
- Maybe let you specify a port instead of defaulting to 53
- Let you configure DNS servers with a YAML file
- Probably a better name like insane-o-flex or something
