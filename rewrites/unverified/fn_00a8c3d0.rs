// original: 0x00a8c3d0 pool_activate_slot (proposed)

/// Activate the record's slot through the two stage callees, then flag it.
///
/// `this` holds the table base at +0xE4 and the entry limit (16-bit) at
/// +0xE8; `rec` carries the slot index at +0x38 and the object at +0x34.
/// A non-positive index returns the incoming register (fixed by the proof);
/// an index at or above the limit returns the limit; otherwise the first
/// stage callee runs on table+8+index*160 and a true answer returns its
/// value. Then the second stage callee runs, and unless the object is null
/// its flag word is cleared of bit 0x20000000, set with bit 0x8000000 and
/// its mode byte set to 5. Returns the object.
///
/// Original: 0x00A8C3D0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a8c3d0(this: u32, rec: u32) -> u32 {
    unsafe {
        const CALLEE_STAGE1: u32 = 1;
        const CALLEE_STAGE2: u32 = 2;
        const TABLE: u32 = 0xe4;
        const LIMIT: u32 = 0xe8;
        const REC_INDEX: u32 = 0x38;
        const REC_OBJ: u32 = 0x34;
        const ENTRY_SIZE: u32 = 160;
        const TABLE_HDR: u32 = 8;
        const FLAG_WORD: u32 = 0x24;
        const MODE_BYTE: u32 = 0x41;
        const CLEAR_BIT: u32 = 0x20000000;
        const SET_BIT: u32 = 0x08000000;
        const MODE_ACTIVE: u8 = 5;
        const INCOMING_EAX: u32 = 0x12345678;
        let index = ((rec + REC_INDEX) as *const i32).read_unaligned();
        if index <= 0 {
            return INCOMING_EAX;
        }
        let limit =
            ((this + LIMIT) as *const u16).read_unaligned() as u32;
        if (index as u32) >= limit {
            return limit;
        }
        let table = ((this + TABLE) as *const u32).read_unaligned();
        let entry = table
            .wrapping_add(TABLE_HDR)
            .wrapping_add((index as u32).wrapping_mul(ENTRY_SIZE));
        let s1 = lf_checker_rt::callee_thiscall!(
            CALLEE_STAGE1,
            u32,
            entry,
            rec
        );
        if s1 as u8 != 0 {
            return s1;
        }
        lf_checker_rt::callee_thiscall!(CALLEE_STAGE2, u32, entry, rec);
        let obj = ((rec + REC_OBJ) as *const u32).read_unaligned();
        if obj == 0 {
            return 0;
        }
        let flags = ((obj + FLAG_WORD) as *mut u32).read_unaligned();
        ((obj + FLAG_WORD) as *mut u32)
            .write_unaligned(flags & !CLEAR_BIT);
        let flags2 = ((obj + FLAG_WORD) as *mut u32).read_unaligned();
        ((obj + FLAG_WORD) as *mut u32).write_unaligned(flags2 | SET_BIT);
        ((obj + MODE_BYTE) as *mut u8).write(MODE_ACTIVE);
        obj
    }
});
