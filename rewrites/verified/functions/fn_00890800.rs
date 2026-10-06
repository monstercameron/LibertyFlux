// original: 0x00890800 rage::audSound::find_by_hash

/// Find a voice object by hash: scan the eight slot ids and return the first
/// non-null slot-0 lookup result.
///
/// `this` points to the sound and `hash` is the key. When the fast-path bit
/// (0x80 at `+0x39`) is set and the kind word at `+0x06` equals 2, or when the
/// disable bit (0x04 at `+0x3a`) is set, the result is null. Otherwise each
/// slot id byte at `+0x48` (eight of them, 0xff meaning empty) forms a cursor:
/// the scalar global times the id (unsigned 32-bit wrap) plus the row word of
/// the voice table global at voice id `[this+0x40]` times the row stride plus
/// the row bias. A null cursor is skipped; otherwise the table slot at `+0x00`
/// of the object at the cursor (callee 1, thiscall) runs with the cursor and
/// the hash, and the first non-null answer is returned. All comparisons are
/// equality tests except the small signed loop bound (`0..8`).
///
/// Original: 0x00890800 (thiscall, one stack word, up to 8 calls).
lf_checker_rt::export!(thiscall, rw_00890800(this: u32, hash: u32) -> u32 {
    unsafe {
        const FAST_FLAG: u32 = 0x39;
        const FAST_BIT: u8 = 0x80;
        const KIND: u32 = 0x06;
        const KIND_FAST: u16 = 2;
        const DISABLE_FLAG: u32 = 0x3a;
        const DISABLE_BIT: u8 = 0x04;
        const VOICE_ID: u32 = 0x40;
        const SLOT_IDS: u32 = 0x48;
        const SLOT_COUNT: u32 = 8;
        const NO_ID: u8 = 0xff;
        const SCALAR_GLOBAL: u32 = 0x0115_d964;
        const VOICE_TABLE: u32 = 0x0115_d988;
        const ROW_STRIDE: u32 = 0x6f40;
        const ROW_BIAS: u32 = 0x6f10;
        const FIND_SLOT: u32 = 0x00;

        if ((this + FAST_FLAG) as *const u8).read() & FAST_BIT != 0
            && ((this + KIND) as *const u16).read_unaligned() == KIND_FAST
        {
            return 0;
        }
        if ((this + DISABLE_FLAG) as *const u8).read() & DISABLE_BIT != 0 {
            return 0;
        }
        let vid = ((this + VOICE_ID) as *const u8).read() as u32;
        let scalar = (lf_checker_rt::relocated(SCALAR_GLOBAL) as *const u32).read_unaligned();
        let tab = (lf_checker_rt::relocated(VOICE_TABLE) as *const u32).read_unaligned();
        let row = ((tab
            .wrapping_add(vid.wrapping_mul(ROW_STRIDE))
            .wrapping_add(ROW_BIAS)) as *const u32)
            .read_unaligned();
        let mut i = 0u32;
        while i < SLOT_COUNT {
            let id = ((this + SLOT_IDS + i) as *const u8).read();
            if id != NO_ID {
                let cursor = scalar.wrapping_mul(id as u32).wrapping_add(row);
                if cursor != 0 {
                    let vtable = (cursor as *const u32).read_unaligned();
                    let find: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(
                        ((vtable + FIND_SLOT) as *const u32).read_unaligned() as usize,
                    );
                    let found = find(cursor, hash);
                    if found != 0 {
                        return found;
                    }
                }
            }
            i += 1;
        }
        0
    }
});
