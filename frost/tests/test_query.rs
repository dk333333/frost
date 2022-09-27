use std::{fs::File, io::Write, path::PathBuf};

use frost::errors::FrostErrorKind;
use frost::query::Query;
use frost::time::Time;
use frost::{msgs::Msg, Bag};
use tempfile::{tempdir, TempDir};

fn write_test_fixture() -> (TempDir, PathBuf) {
    let bytes = include_bytes!("fixtures/test.bag");

    let tmp_dir = tempdir().unwrap();
    let file_path = tmp_dir.path().join("test.bag");
    {
        let mut tmp_file = File::create(file_path.clone()).unwrap();
        tmp_file.write(bytes).unwrap();
    }
    (tmp_dir, file_path)
}

#[test]
fn bag_iter() {
    let (_tmp_dir, file_path) = write_test_fixture();
    let mut bag = Bag::from(file_path).unwrap();

    let query = Query::all();
    let count = bag.read_messages(&query).count();
    assert_eq!(count, 200);

    let query = Query::new().with_topics(&vec!["/chatter"]);
    let count = bag.read_messages(&query).count();
    assert_eq!(count, 100);
}

// these are technically the wrong types for loadig the messages (not coming from ros .msgs),
// but we're not using codegen on the std_msgs for the lib,
// and serde_rosmsg is able to handle the conversion
#[derive(Clone, Debug, serde_derive::Deserialize, PartialEq)]
struct NewString(String);
#[derive(Clone, Debug, serde_derive::Deserialize, PartialEq)]
struct NewTime(Time);
impl Msg for NewString {}
impl Msg for NewTime {}

#[test]
fn msg_reading() {
    let (_tmp_dir, file_path) = write_test_fixture();
    let mut bag = Bag::from(file_path).unwrap();

    let query = Query::new().with_topics(&["/chatter"]);
    let count = bag.read_messages(&query).count();
    assert_eq!(count, 100);

    for (i, msg_view) in bag.read_messages(&query).enumerate() {
        let msg = msg_view.instantiate::<NewString>().unwrap();
        assert_eq!(msg.0, format!("foo_{i}"))
    }

    let query = Query::new().with_topics(&["/time"]);
    let count = bag.read_messages(&query).count();
    assert_eq!(count, 100);

    for (i, msg_view) in bag.read_messages(&query).enumerate() {
        let msg = msg_view.instantiate::<NewTime>().unwrap();
        assert_eq!(msg.0.secs, i as u32)
    }
}

#[test]
fn msg_reading_wrong_type() {
    let (_tmp_dir, file_path) = write_test_fixture();
    let mut bag = Bag::from(file_path).unwrap();

    let query = Query::new().with_topics(&["/chatter"]);
    let msg_view = bag.read_messages(&query).last().unwrap();

    // Try to read a string as a Time
    let res = msg_view.instantiate::<NewTime>();
    assert!(matches!(
        res.unwrap_err().kind(),
        FrostErrorKind::Deserialization(_)
    ))
}