use crate::models::properties::{RollupFunction, RelationValue};

use super::{DateOrDateTime, PropertyValue, RollupPropertyValue, RollupValue};
use chrono::NaiveDate;

#[test]
fn verify_date_parsing() {
    let date = NaiveDate::from_ymd_opt(2021, 01, 02).unwrap();
    let result = serde_json::to_string(&DateOrDateTime::Date(date)).unwrap();
    let parsed: DateOrDateTime = serde_json::from_str(&result).unwrap();
    println!("{:?}", parsed);
}

#[test]
fn parse_date_property() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/date_property.json")).unwrap();
}

#[test]
fn parse_null_select_property() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/null_select_property.json")).unwrap();
}

#[test]
fn parse_select_property() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/select_property.json")).unwrap();
        assert!(matches!(_property, 
            PropertyValue::Select { select: Some(..), .. }));
}

#[test]
fn parse_select_empty_property() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/select_empty_property.json")).unwrap();
        assert!(matches!(_property, 
            PropertyValue::Select { select: None, .. }));
}

#[test]
fn parse_text_property_with_link() {
    let _property: PropertyValue =
        serde_json::from_str(include_str!("tests/text_with_link.json")).unwrap();
}

#[test]
fn parse_relation_property() {
    let property: PropertyValue =
    serde_json::from_str(include_str!("tests/relation_property.json")).unwrap();

    assert!(matches!(
        property,
        PropertyValue::Relation { .. }
    ));

    if let PropertyValue::Relation {
        relation,
        ..
    } = &property
    {
        assert!(matches!(relation[0], RelationValue{ .. }))
    }
}

#[test]
fn parse_rollup_property() {
    let property: PropertyValue =
        serde_json::from_str(include_str!("tests/rollup_property.json")).unwrap();

    assert!(matches!(
        property,
        PropertyValue::Rollup {
            rollup: RollupValue::Array { .. },
            ..
        }
    ));

    if let PropertyValue::Rollup {
        rollup: RollupValue::Array { array , function: RollupFunction::ShowOriginal },
        ..
    } = property
    {
        assert!(matches!(array[0], RollupPropertyValue::Text { .. }))
    }
}


#[test]
fn parse_rollup_number_property() {
    let property: PropertyValue =
        serde_json::from_str(include_str!("tests/rollup_number_property.json")).unwrap();

    assert!(matches!(
        property,
        PropertyValue::Rollup {
            rollup: RollupValue::Number { .. },
            ..
        }
    ));
}
