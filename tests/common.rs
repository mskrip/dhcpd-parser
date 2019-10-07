extern crate dhcpd_parser;

use crate::dhcpd_parser::common::Date;

#[test]
fn date_rfc3339() {
    assert_eq!(
        Date::from_rfc3339(1, "2019-03-01T00:00:00+00:00").unwrap(),
        Date {
            weekday: 1,
            year: 2019,
            month: 3,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
        }
    );

    assert_eq!(
        Date::from_rfc3339(6, "2015-01-01T21:21:21Z").unwrap(),
        Date {
            weekday: 6,
            year: 2015,
            month: 1,
            day: 1,
            hour: 21,
            minute: 21,
            second: 21,
        }
    );

    assert_eq!(
        Date::from_rfc3339(7, "2015-01-01T21:21:21Z").unwrap_err(),
        "Weekday should be a number between 0 and 6. 7 is not",
    );
    assert_eq!(
        Date::from_rfc3339(1, "T").unwrap_err(),
        "This doesn\'t seem like a correct RFC3339 date: \"T\"",
    );
}
