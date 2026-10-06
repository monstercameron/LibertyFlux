// original: 0x00B54750 rage::crmtFrameBufferFixed<220>::vf1

/// Allocate one 20-byte slot from the frame buffer's free bitmap.
///
/// Holds the buffer lock (callee 1 takes a frame guard slot plus the
/// address `this` + 0x3F4; callee 2 releases the guard) while scanning the
/// 50-bit bitmap at `this` + 0x3EC for the first clear bit, comparing the
/// index against 50 unsigned. The bit is set and the address of slot
/// `this` + 4 + index * 20 is returned; when every bit is set the result
/// is zero. The guard slot address differs per side and is skipped in
/// favour of a snapshot of its two words.
///
/// Original: 0x00B54750 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00b54750(this: u32) -> u32 {
    unsafe {
        const BITSET: u32 = 0x3ec;
        const LOCK_ARG: u32 = 0x3f4;
        const SLOTS: u32 = 50;
        const SLOT_STRIDE: u32 = 20;
        const LOCK_INIT: u32 = 1;
        const LOCK_FREE: u32 = 2;
        let mut guard = [0u32; 2];
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOCK_INIT, u32, (&mut guard as *mut u32) as u32, this + LOCK_ARG
        );
        let mut idx = 0u32;
        let found = loop {
            let bit = 1u32 << (idx & 31);
            let cell = (this + BITSET).wrapping_add((idx >> 5).wrapping_mul(4));
            let word = (cell as *const u32).read_unaligned();
            (cell as *mut u32).write_unaligned(word | bit);
            if (word & bit) == 0 {
                break true;
            }
            idx += 1;
            if idx >= SLOTS {
                break false;
            }
        };
        let out = if found {
            (this + 4).wrapping_add(idx.wrapping_mul(SLOT_STRIDE))
        } else {
            0
        };
        let _: u32 = lf_checker_rt::callee_thiscall!(
            LOCK_FREE, u32, (&mut guard as *mut u32) as u32
        );
        out
    }
});
