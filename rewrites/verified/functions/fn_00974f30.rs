// original: 0x00974f30 audio_marked_slot_acquire (proposed)

/// Acquire the next 0x80-marked slot of the circular array.
///
/// Under the lock at +0x8 the cursor at +0x30 advances (wrapping at the
/// count in +0x28); a full lap with no marked byte releases the lock and
/// returns 0. On a marked byte the mark is cleared, the pending count at
/// +0x2C drops, the lock releases and the slot address (base at +0x0 plus
/// row times 384) returns. Equality compares only.
/// Original: 0x00974F30 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00974f30(this: u32) -> u32 {
    unsafe {
        const LOCK: u32 = 8;
        const BASE: u32 = 0;
        const ARR: u32 = 4;
        const COUNT: u32 = 0x28;
        const PENDING: u32 = 0x2C;
        const CURSOR: u32 = 0x30;
        const MARK: u8 = 0x80;
        const ROW: u32 = 384;
        const ACQUIRE: u32 = 1;
        const RELEASE: u32 = 2;
        lf_checker_rt::callee_thiscall!(ACQUIRE, u32, this.wrapping_add(LOCK));
        let count = ((this.wrapping_add(COUNT)) as *const u32).read_unaligned();
        let mut wrapped = false;
        loop {
            let cur = ((this.wrapping_add(CURSOR)) as *const u32)
                .read_unaligned()
                .wrapping_add(1);
            ((this.wrapping_add(CURSOR)) as *mut u32).write_unaligned(cur);
            let i = if cur == count {
                ((this.wrapping_add(CURSOR)) as *mut u32).write_unaligned(0);
                if wrapped {
                    lf_checker_rt::callee_thiscall!(RELEASE, u32, this.wrapping_add(LOCK));
                    return 0;
                }
                wrapped = true;
                0
            } else {
                cur
            };
            let arr = ((this.wrapping_add(ARR)) as *const u32).read_unaligned();
            let cell = (arr.wrapping_add(i)) as *mut u8;
            let b = cell.read();
            if b & MARK != 0 {
                cell.write(b & !MARK);
                let n = ((this.wrapping_add(PENDING)) as *const u32).read_unaligned();
                ((this.wrapping_add(PENDING)) as *mut u32).write_unaligned(n.wrapping_sub(1));
                lf_checker_rt::callee_thiscall!(RELEASE, u32, this.wrapping_add(LOCK));
                let base = ((this.wrapping_add(BASE)) as *const u32).read_unaligned();
                return base.wrapping_add(i.wrapping_mul(ROW));
            }
        }
    }
});
