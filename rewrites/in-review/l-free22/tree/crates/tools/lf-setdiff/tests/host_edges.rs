//! Host tests of the lifted field store and its instance table. These run on every host and do not need
//! the 32-bit rewrites.

use lf_core::field_store::instances::STORES;
use lf_core::field_store::{FieldStore, FieldWidth, ObjectBytes, StoreResult};

#[test]
fn the_table_lists_fifty_instances() {
    assert_eq!(STORES.len(), 50);
}

#[test]
fn every_instance_names_a_symbol_and_fits_a_large_object() {
    for named in STORES.iter() {
        assert!(!named.name.is_empty());
        let width = match named.store.width {
            FieldWidth::Byte => 1,
            FieldWidth::Word => 4,
        };
        assert!(named.store.offset + width <= 0x2000, "{}", named.name);
    }
}

#[test]
fn no_two_instances_share_a_name() {
    let mut names: Vec<&str> = STORES.iter().map(|named| named.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), STORES.len());
}

#[test]
fn a_byte_instance_writes_one_byte_and_nothing_else() {
    let byte_instances = STORES
        .iter()
        .filter(|named| named.store.width == FieldWidth::Byte);
    for named in byte_instances {
        let mut object = ObjectBytes::zeroed(0x2000);
        let before = object.clone();
        let _ = object.store(named.store, 0xA5_5A_0F_F0);
        let changed: Vec<usize> = (0..object.len())
            .filter(|&at| object.byte(at) != before.byte(at))
            .collect();
        assert!(changed.len() <= 1, "{}", named.name);
        assert_eq!(object.byte(named.store.offset), 0xF0, "{}", named.name);
    }
}

#[test]
fn a_word_instance_writes_four_bytes_at_its_offset() {
    let word_instances = STORES
        .iter()
        .filter(|named| named.store.width == FieldWidth::Word);
    for named in word_instances {
        let mut object = ObjectBytes::zeroed(0x2000);
        let _ = object.store(named.store, 0x1122_3344);
        assert_eq!(object.word(named.store.offset), 0x1122_3344, "{}", named.name);
    }
}

#[test]
fn result_kinds_map_the_parameter_as_documented() {
    let mut object = ObjectBytes::zeroed(8);
    let zero = FieldStore {
        offset: 0,
        width: FieldWidth::Word,
        result: StoreResult::Zero,
    };
    let value = FieldStore {
        result: StoreResult::Value,
        ..zero
    };
    let low = FieldStore {
        result: StoreResult::LowByte,
        ..zero
    };
    let nothing = FieldStore {
        result: StoreResult::Nothing,
        ..zero
    };
    assert_eq!(object.store(zero, 0xDEAD_BEEF), Some(0));
    assert_eq!(object.store(value, 0xDEAD_BEEF), Some(0xDEAD_BEEF));
    assert_eq!(object.store(low, 0xDEAD_BEEF), Some(0xEF));
    assert_eq!(object.store(nothing, 0xDEAD_BEEF), None);
}

