//! Host tests of the constant-write instance table and the lifted routine. These run on every host and do not
//! need the 32-bit rewrites.

use lf_core::constant_writes::instances::INSTANCES;
use lf_core::constant_writes::{ConstantWrite, Returns, WriteWidth, apply_writes};
use lf_core::field_store::ObjectBytes;

#[test]
fn the_table_lists_ten_instances() {
    assert_eq!(INSTANCES.len(), 10);
}

#[test]
fn no_two_instances_share_a_name() {
    let mut names: Vec<&str> = INSTANCES.iter().map(|named| named.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), INSTANCES.len());
}

#[test]
fn every_instance_writes_inside_a_large_object() {
    for named in INSTANCES.iter() {
        for write in named.writes.writes {
            let width = match write.width {
                WriteWidth::Byte => 1,
                WriteWidth::Half => 2,
                WriteWidth::Word => 4,
            };
            assert!(write.offset + width <= 0x2000, "{}", named.name);
        }
    }
}

#[test]
fn a_half_word_write_keeps_its_low_sixteen_bits() {
    let writes = [ConstantWrite {
        offset: 1,
        width: WriteWidth::Half,
        value: 0xFFFF_1234,
    }];
    let mut object = ObjectBytes::zeroed(4);
    assert_eq!(apply_writes(&writes, Returns::Zero, &mut object, 0), 0);
    assert_eq!(object.bytes(), &[0, 0x34, 0x12, 0]);
}

#[test]
fn this_returns_the_address_it_is_given() {
    let mut object = ObjectBytes::zeroed(4);
    assert_eq!(apply_writes(&[], Returns::This, &mut object, 0xDEAD_0008), 0xDEAD_0008);
}
