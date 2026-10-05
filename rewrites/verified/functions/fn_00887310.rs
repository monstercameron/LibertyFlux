// original: 0x00887310 stream_slot_acquire (proposed)

/// Take the next free slot of the stream-slot table under its lock.
///
/// Locks (callee 1) with `this + 8`, then scans the flag bytes at the
/// array `[this+4]` from index `[this+0x30]`, wrapping at the count
/// `[this+0x28]` for at most two passes. The first byte with bit 0x80 set
/// has the bit cleared, the free counter at `[this+0x2c]` is decremented,
/// and the slot address `base + index * 0x4f0` (from `[this]`) is returned
/// after unlocking (callee 2). When no byte is set, unlocks and returns 0.
///
/// Original: 0x00887310 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00887310(this: u32) -> u32 {
    unsafe {
        const LOCK_OFF: u32 = 8;
        const BASE: u32 = 0x00;
        const FLAGS: u32 = 0x04;
        const COUNT: u32 = 0x28;
        const FREE: u32 = 0x2c;
        const CURSOR: u32 = 0x30;
        const TAKEN: u8 = 0x80;
        const SLOT_STRIDE: u32 = 0x4f0;
        const LOCK: u32 = 1;
        const UNLOCK: u32 = 2;
        let w = this.wrapping_add(LOCK_OFF);
        lf_checker_rt::callee_thiscall!(LOCK, u32, w);
        let count = ((this + COUNT) as *const u32).read_unaligned();
        let mut wrapped = false;
        loop {
            let idx = ((this + CURSOR) as *const u32)
                .read_unaligned()
                .wrapping_add(1);
            ((this + CURSOR) as *mut u32).write_unaligned(idx);
            if idx == count {
                ((this + CURSOR) as *mut u32).write_unaligned(0);
                if wrapped {
                    lf_checker_rt::callee_thiscall!(UNLOCK, u32, w);
                    return 0;
                }
                wrapped = true;
            }
            let arr = ((this + FLAGS) as *const u32).read_unaligned();
            let i = ((this + CURSOR) as *const u32).read_unaligned();
            let p = (arr.wrapping_add(i)) as *mut u8;
            if p.read() & TAKEN != 0 {
                p.write(p.read() & !TAKEN);
                let f = ((this + FREE) as *const u32).read_unaligned();
                ((this + FREE) as *mut u32)
                    .write_unaligned(f.wrapping_sub(1));
                lf_checker_rt::callee_thiscall!(UNLOCK, u32, w);
                let base = ((this + BASE) as *const u32).read_unaligned();
                return base.wrapping_add(i.wrapping_mul(SLOT_STRIDE));
            }
        }
    }
});
