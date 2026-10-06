// original: 0x00dcf1b0 csv_next_char (proposed)

/// Advance the csv reader by one character and return the new current char.
///
/// `this` points to the reader. The current char at `+0x418` is first copied
/// to the previous slot at `+0x419`. On the fast path (fill flag at `+0x40c`
/// set and position at `+0x414` below length at `+0x410`, compared SIGNED)
/// the position is incremented and the char at the new position returned.
/// Otherwise the buffer is refilled: the fill callee reads 0x400 bytes
/// through the handle at `+0x0` into the buffer at `+0xc` (cdecl, three
/// words), the base callee's answer is stored at `+0x8` (cdecl, one word),
/// the flag is set, the buffer's strlen becomes the length, the position is
/// reset to zero and the first char returned.
///
/// Original: 0x00DCF1B0 (thiscall, no stack arguments, byte result).
lf_checker_rt::export!(thiscall, rw_00dcf1b0(this: u32) -> u32 {
    unsafe {
        /// Refill-read callee id: (handle, buffer, capacity).
        const FILL: u32 = 1;
        /// Base-offset callee id: (handle).
        const BASE: u32 = 2;
        /// Fill capacity passed to the refill callee.
        const CAP: u32 = 0x400;
        const FLAG: u32 = 0x40c;
        const POS: u32 = 0x414;
        const LEN: u32 = 0x410;
        const CUR: u32 = 0x418;
        const PREV: u32 = 0x419;
        const BUF: u32 = 0xc;
        let cur = ((this + CUR) as *const u8).read();
        ((this + PREV) as *mut u8).write(cur);
        let filled = ((this + FLAG) as *const u8).read() != 0;
        if filled {
            // SIGNED compare (jge): pos - len >= 0 refills.
            let pos = ((this + POS) as *const i32).read_unaligned();
            let len = ((this + LEN) as *const i32).read_unaligned();
            if pos < len {
                let np = pos.wrapping_add(1);
                ((this + POS) as *mut i32).write_unaligned(np);
                let c = ((this.wrapping_add(np as u32).wrapping_add(BUF)) as *const u8).read();
                ((this + CUR) as *mut u8).write(c);
                return c as u32;
            }
        }
        let handle = (this as *const u32).read_unaligned();
        let buf = this.wrapping_add(BUF);
        lf_checker_rt::callee_cdecl!(FILL, u32, handle, buf, CAP);
        let base: u32 = lf_checker_rt::callee_cdecl!(BASE, u32, handle);
        ((this + 8) as *mut u32).write_unaligned(base);
        ((this + FLAG) as *mut u8).write(1);
        // strlen of the refilled buffer.
        let mut n: u32 = 0;
        if buf != 0 {
            let mut p = buf;
            loop {
                let b = (p as *const u8).read();
                p = p.wrapping_add(1);
                if b == 0 {
                    break;
                }
                n = n.wrapping_add(1);
            }
        }
        ((this + POS) as *mut u32).write_unaligned(0);
        ((this + LEN) as *mut u32).write_unaligned(n);
        let c = (buf as *const u8).read();
        ((this + CUR) as *mut u8).write(c);
        c as u32
    }
});
