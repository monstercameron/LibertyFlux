// original: 0x00D99330 CBoat::vf43

/// Refresh boat state through the base helper. When the attached entity is
/// non-null, copy its selected transform and status words, notify the entity
/// helper, and set bit 3 in the indexed table byte. When the vehicle flag at
/// `+0x1310` contains bit 2, call the local state helper with two one-valued
/// arguments. The global pool and table roots are read from relocated cells.
///
/// This is a 32-bit thiscall with no stack arguments. The three outgoing
/// helpers are modeled by checker call stubs; the entity is at `+0x167c`, and
/// its two-byte table index is at `+8`.
use crate::support::{read_u8, read_u16, read_u32, write_u8, write_u32};

lf_checker_rt::export!(thiscall, rw_00D99330(this: u32) -> () {
    const BASE_UPDATE: u32 = 6;
    const ENTITY_NOTIFY: u32 = 7;
    const LOCAL_STATE: u32 = 8;
    const ENTITY_POINTER: u32 = 0x167C;
    const SOURCE_POINTER: u32 = 0x20;
    const VEHICLE_FLAGS: u32 = 0x1310;
    const TABLE_ROOT_VA: u32 = 0x012B_9C78;
    const POOL_ROOT_VA: u32 = 0x012B_9C7C;
    const TABLE_POINTER_OFFSET: u32 = 0x34;
    const INDEX_OFFSET: u32 = 8;
    const BIT_TO_SET: u8 = 8;
    const COPY_FIELDS: [(u32, u32); 12] = [
        (0x10, 0), (0x14, 4), (0x18, 8), (0x20, 0x10),
        (0x24, 0x14), (0x28, 0x18), (0x30, 0x20), (0x34, 0x24),
        (0x38, 0x28), (0x40, 0x30), (0x44, 0x34), (0x48, 0x38),
    ];
    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(BASE_UPDATE, u32, this);
        let entity = read_u32(this.wrapping_add(ENTITY_POINTER));
        if entity != 0 {
            let source = read_u32(this.wrapping_add(SOURCE_POINTER));
            for (destination_offset, source_offset) in COPY_FIELDS {
                let value = read_u32(source.wrapping_add(source_offset));
                write_u32(entity.wrapping_add(destination_offset), value);
            }
            let pool = lf_checker_rt::global::<u32>(POOL_ROOT_VA).read_unaligned();
            let _ = lf_checker_rt::callee_thiscall!(ENTITY_NOTIFY, u32, pool, entity, 0);
            let index = read_u16(entity.wrapping_add(INDEX_OFFSET)) as u32;
            let table_root = lf_checker_rt::global::<u32>(TABLE_ROOT_VA).read_unaligned();
            let table = read_u32(table_root.wrapping_add(TABLE_POINTER_OFFSET));
            let slot = table.wrapping_add(index);
            let old_value = read_u8(slot);
            write_u8(slot, old_value | BIT_TO_SET);
        }
        if read_u8(this.wrapping_add(VEHICLE_FLAGS)) & 4 != 0 {
            let _ = lf_checker_rt::callee_thiscall!(LOCAL_STATE, u32, this, 1u32, 1u32);
        }
    }
});
