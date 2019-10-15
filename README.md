# dhcpd config parser

Rust library for parsing OpenBSD dhcpd configuration files.

The library currently supports only OpenBSD implementation of the
`dhcpd.leases` file format. See
[man pages](https://man.openbsd.org/dhcpd.leases.5)

## Example usage

```rust
use dhcpd_parser::parser;
use dhcpd_parser::parser::LeasesMethods;


let res = parser::parse("
    lease 192.168.0.2 {
        starts 2 2019/01/01 22:00:00 UTC;
        ends 2 2019/01/01 22:00:00 UTC;
        hardware type 11:11:11:11:11:11;
        uid Client1;
        client-hostname \"CLIENTHOSTNAME\";
        hostname \"TESTHOSTNAME\";
        abandoned;
    }

    lease 192.168.0.3 {
        starts 1 1985/01/01 00:00:00 UTC;
        hardware type 22:22:22:22:22:22;
        uid Client2;
        hostname \"TESTHOSTNAME\";
    }
".to_string()).expect("This should be a correct lease file");

let leases = res.unwrap().leases;

assert_eq!(
    leases[0].hostname.as_ref().unwrap(),
    "TESTHOSTNAME",
);
assert_eq!(
    leases[1].dates.starts.unwrap().to_string(),
    "Monday 1985/01/01 00:00:00",
);
assert!(leases[1].dates.ends.is_none());

assert!(leases[0].abandoned);
assert!(!leases[1].abandoned);
assert_eq!(
    leases.by_leased("192.168.0.2".to_string()).unwrap(),
    leases[0],
);
assert_eq!(
    leases.client_hostnames(),
    ["CLIENTHOSTNAME".to_owned()]
        .iter()
        .cloned()
        .collect(),
);
```
