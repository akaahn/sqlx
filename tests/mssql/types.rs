use sqlx::mssql::Mssql;
use sqlx_test::test_type;

test_type!(null<Option<i32>>(Mssql,
    "CAST(NULL as INT)" == None::<i32>
));

test_type!(i8(
    Mssql,
    "CAST(5 AS TINYINT)" == 5_i8,
    "CAST(0 AS TINYINT)" == 0_i8
));

test_type!(i16(Mssql, "CAST(21415 AS SMALLINT)" == 21415_i16));

test_type!(i32(Mssql, "CAST(2141512 AS INT)" == 2141512_i32));

test_type!(i64(Mssql, "CAST(32324324432 AS BIGINT)" == 32324324432_i64));

test_type!(f32(
    Mssql,
    "CAST(3.1410000324249268 AS REAL)" == 3.141f32 as f64 as f32
));

test_type!(f64(
    Mssql,
    "CAST(939399419.1225182 AS FLOAT)" == 939399419.1225182_f64
));

test_type!(str_nvarchar<String>(Mssql,
    "CAST('this is foo' as NVARCHAR)" == "this is foo",
));

test_type!(str<String>(Mssql,
    "'this is foo'" == "this is foo",
    "''" == "",
));

test_type!(bool(
    Mssql,
    "CAST(1 as BIT)" == true,
    "CAST(0 as BIT)" == false
));

test_type!(uuid<sqlx::types::Uuid>(Mssql,
    "CAST('b731678f-636f-4135-bc6f-19440c13bd19' AS uniqueidentifier)"
        == sqlx::types::Uuid::parse_str("b731678f-636f-4135-bc6f-19440c13bd19").unwrap(),
    "CAST('00000000-0000-0000-0000-000000000000' AS uniqueidentifier)"
        == sqlx::types::Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
));

test_type!(chrono_date<NaiveDate>(MySql,
    "CAST('1999-03-04 23:17:00.000' AS DATETIME)" == NaiveDate::from_ymd(1999, 03, 04).and_hms(23, 17, 0)
    "CAST('2021-10-29 13:48:53.000' AS DATETIME)" == NaiveDate::from_ymd(2021, 10, 29).and_hms(13, 48, 53),
));
