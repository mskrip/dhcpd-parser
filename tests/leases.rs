extern crate dhcpd_parser;

use crate::dhcpd_parser::parser;
use crate::dhcpd_parser::parser::LeasesMethods;

#[test]
fn basic_test() {
    let res = parser::parse("
    lease 192.0.0.2 {

    }".to_string());
    assert!(res.is_ok());
}

#[test]
fn dates_test() {
    let res = parser::parse("lease 255.254.253.252 {
        starts 2 2019/01/01 22:00:00
        ends 2 2019/01/01 22:00:00
    }".to_string());
    assert!(res.is_ok());
}

#[test]
fn all_options_test() {
    let res = parser::parse("
    lease 192.168.0.2 {
        starts 2 2019/01/01 22:00:00
        ends 2 2019/01/01 22:00:00
        hardware type 11:11:11:11:11:11
        uid Client1
        client-hostname CLIENTHOSTNAME
        hostname TESTHOSTNAME
        abandoned
    }".to_string());

    assert!(res.is_ok());
}

#[test]
fn multiple_leases_test() {
    let res = parser::parse("
    lease 192.168.0.2 {
        starts 2 2019/01/01 22:00:00
        ends 2 2019/01/01 22:00:00
        hardware type 11:11:11:11:11:11
        uid Client1
        client-hostname CLIENTHOSTNAME
        hostname TESTHOSTNAME
        abandoned
    }

    lease 192.168.0.3 {
        starts 1 1985/01/01 00:00:00
        hardware type 22:22:22:22:22:22
        uid Client2
        hostname TESTHOSTNAME
    }
    ".to_string());

    assert!(res.is_ok());

    let leases = res.unwrap().leases;
    assert_eq!(leases[0].hostname.as_ref().unwrap(), "TESTHOSTNAME");
    assert_eq!(leases[1].dates.starts.unwrap().to_string(), "Monday 1985/01/01 00:00:00");
    assert!(leases[1].dates.ends.is_none());

    assert!(leases[0].abandoned);
    assert!(!leases[1].abandoned);

    assert_eq!(leases.by_leased("192.168.0.2".to_string()).unwrap(), leases[0]);
}

#[test]
fn invalid_format_test () {
    let res = parser::parse("
    lease 192.0.0.2 {

    ".to_string());
    assert!(res.is_err());
}

#[test]
fn invalid_date_format_test () {
    let res = parser::parse("
    lease 192.0.0.2 {
        starts 2 2019-01-02T00:00:00Z
    }".to_string());
    assert!(res.is_err());
}
