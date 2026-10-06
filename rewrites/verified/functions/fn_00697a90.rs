// original: 0x00697a90 pool_slot_rebind (proposed)

/// Rebind a pool slot to this table, looking up `key` by binary search.
///
/// If the slot's current entry (`slot+4`) is set, its table's mutex (owner at
/// `slot+0`, mutex word at `+0x10`) is waited on when non-zero, the old
/// entry's refcount (`+8`) is decremented, the slot is cleared, and the mutex
/// released. Then the slot's owner becomes `this` and `key` is sought in the
/// table's unsigned-sorted entry array (`entries` at `this+4`, `count` a
/// 16-bit half at `this+8`) with a signed `lo`/`hi` cursor and an unsigned
/// key comparison: equal returns the entry, an entry key at or above `key`
/// moves `hi` down, otherwise `lo` moves up. On a hit the table generation
/// (`this+0`) is incremented, the entry is stamped with it (`+4`), its
/// refcount (`+8`) incremented, and the slot pointed at it. Returns 1 on a
/// hit, 0 on a miss. Only the low result byte is significant: on the
/// empty-table path the upper bytes keep whatever `eax` held on entry.
///
/// Original: thiscall, two stack words (`key`, `slot`), callee cleans 8.
lf_checker_rt::export!(thiscall, rw_00697a90(this: u32, key: u32, slot: u32) -> u32 {
    unsafe {
        const WAIT: u32 = 10;
        const RELEASE: u32 = 11;
        const INFINITE: u32 = 0xffff_ffff;
        const OWNER_MUTEX: u32 = 0x10;
        const ENTRY_STAMP: u32 = 4;
        const ENTRY_REFCOUNT: u32 = 8;
        const TABLE_ENTRIES: u32 = 4;
        const TABLE_COUNT: u32 = 8;

        let slot_p = slot as *mut u32;
        if slot_p.byte_offset(4).read_unaligned() != 0 {
            let owner = slot_p.read_unaligned();
            let mutex = (owner as *const u32).byte_offset(OWNER_MUTEX as isize).read_unaligned();
            if mutex != 0 {
                let _ = lf_checker_rt::callee_stdcall!(WAIT, u32, mutex, INFINITE);
            }
            let old = slot_p.byte_offset(4).read_unaligned();
            if old != 0 {
                let rc = (old as *const u32).byte_offset(ENTRY_REFCOUNT as isize).read_unaligned();
                (old as *mut u32)
                    .byte_offset(ENTRY_REFCOUNT as isize)
                    .write_unaligned(rc.wrapping_sub(1));
                slot_p.byte_offset(4).write_unaligned(0);
            }
            let owner2 = slot_p.read_unaligned();
            let mutex2 =
                (owner2 as *const u32).byte_offset(OWNER_MUTEX as isize).read_unaligned();
            if mutex2 != 0 {
                let _ = lf_checker_rt::callee_stdcall!(RELEASE, u32, mutex2);
            }
        }
        slot_p.write_unaligned(this);
        let count = (this as *const u16).byte_offset(TABLE_COUNT as isize).read_unaligned() as i32;
        let entries = (this as *const u32).byte_offset(TABLE_ENTRIES as isize).read_unaligned();
        let mut lo: i32 = 0;
        let mut hi: i32 = count - 1;
        if hi < 0 {
            return 0;
        }
        loop {
            let mid = (hi + lo) >> 1;
            let entry = (entries as *const u32).byte_offset((mid * 4) as isize).read_unaligned();
            let entry_key = (entry as *const u32).read_unaligned();
            if entry_key == key {
                let generation =
                    (this as *const u32).read_unaligned().wrapping_add(1);
                (this as *mut u32).write_unaligned(generation);
                (entry as *mut u32).byte_offset(ENTRY_STAMP as isize).write_unaligned(generation);
                if entry != slot_p.byte_offset(4).read_unaligned() {
                    let rc = (entry as *const u32)
                        .byte_offset(ENTRY_REFCOUNT as isize)
                        .read_unaligned();
                    (entry as *mut u32)
                        .byte_offset(ENTRY_REFCOUNT as isize)
                        .write_unaligned(rc.wrapping_add(1));
                    let cur = slot_p.byte_offset(4).read_unaligned();
                    if cur != 0 {
                        let rc2 = (cur as *const u32)
                            .byte_offset(ENTRY_REFCOUNT as isize)
                            .read_unaligned();
                        (cur as *mut u32)
                            .byte_offset(ENTRY_REFCOUNT as isize)
                            .write_unaligned(rc2.wrapping_sub(1));
                    }
                    slot_p.byte_offset(4).write_unaligned(entry);
                }
                return 1;
            } else if entry_key >= key {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
            if lo > hi {
                return 0;
            }
        }
    }
});
