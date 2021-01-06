mod utils;
use bigdecimal::BigDecimal;
use eyre::Result;
use ion_binary_rs::{IonValue, NullIonValue};
use std::collections::HashMap;
use std::convert::TryInto;
use std::str::FromStr;
use utils::create_type_test;

#[async_std::test]
async fn qldb_type_big_struct() -> Result<()> {
    create_type_test(build_big_struct(), |values| {
        let expected_result: HashMap<String, IonValue> = build_big_struct().try_into().unwrap();

        assert_eq!(values, expected_result);
    })
    .await
}

fn build_big_struct() -> IonValue {
    let list = IonValue::List(vec![
        IonValue::Integer(1),
        IonValue::Integer(2),
        IonValue::Integer(3),
        IonValue::Integer(-3),
        IonValue::Integer(-3354654),
    ]);

    let qldb_struct = IonValue::Struct(hashmap!(
        "Model".to_string() => IonValue::String("CLK 350".to_string()),
        "Type".to_string() => IonValue::String("Sedan".to_string()),
        "Color".to_string() => IonValue::String("White".to_string()),
        "VIN".to_string() => IonValue::String("1C4RJFAG0FC625797".to_string()),
        "Make".to_string() => IonValue::String("Mercedes".to_string()),
        "Year".to_string() => IonValue::Integer(2019)
    ));

    let long_struct = IonValue::Struct(hashmap!(
        "000021i".into() => IonValue::Integer(9),
        "012i".into() => IonValue::Integer(9),
        "01d".into() => IonValue::Integer(4),
        "01h".into() => IonValue::Integer(8),
        "11n".into() => IonValue::Float(std::f64::MIN),
        "12l".into() => IonValue::Integer(12),
        "1d".into() => IonValue::Integer(4),
        "21l".into() => IonValue::Integer(12),
        "2h".into() => list,
        "aaa".into() => IonValue::Integer(1),
        "aak".into() => IonValue::Integer(11),
        "ae".into() => IonValue::Integer(5),
        "b".into() => qldb_struct,
        "bb".into() => IonValue::Integer(2),
        "cb".into() => IonValue::Integer(2),
        "c".into() => IonValue::Integer(3),
        "d".into() => IonValue::Null(NullIonValue::Clob),
        "9f".into() => IonValue::Integer(6),
        "09f".into() => IonValue::Decimal(BigDecimal::from_str("92407156491786485918754613897564897561387954629341564305176435762934857629384756024751649587623498561204576329654.1239476129586128957624351682956187465187324618724691845696216935").unwrap()),
        "g".into() => IonValue::Integer(7),
        "00h".into() => IonValue::Integer(8),
        "0h".into() => IonValue::Integer(8),
        "i".into() => IonValue::Integer(9),
        "j".into() => IonValue::Integer(10),
        "k".into() => IonValue::Null(NullIonValue::Float),
        "00001l".into() => IonValue::Integer(12),
        "00002l".into() => IonValue::Integer(12),
        "10000l".into() => IonValue::Integer(12),
        "l".into() => IonValue::Integer(12),
        "m".into() => IonValue::Integer(13),
        "n".into() => IonValue::Integer(14)
    ));

    IonValue::Struct(hashmap!(
        "e".into() => IonValue::Integer(5),
        "a".into() => long_struct,
        "l".into() => IonValue::Integer(12),
        "b".into() => IonValue::Integer(2),
        "i".into() => IonValue::Integer(9),
        "n".into() => IonValue::Float(123.12)
    ))
}

#[macro_export]
macro_rules! hashmap(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);
