// original: 0x0066F1F0 sub_0066f1f0

/// Initialise a base tokenizer object with its stream and flags.
///
/// `this` points to the object, `stream` to its backing stream object and
/// `flags` to the initial flag word. The object stores `stream` at `+0x04`
/// and `flags` at `+0x0C`, then fixed defaults: 1 at `+0x08`, 0x20 at
/// `+0x10`, 0 at `+0x18`, a zero byte at `+0x1C`, 0 at `+0x21C` and 2 at
/// `+0x14`. The return register is untouched, so the contract does not
/// compare it.
///
/// Original: 0x0066F1F0 (thiscall, two stack arguments, no calls).
lf_checker_rt::export!(thiscall, rw_0066f1f0(this: u32, stream: u32, flags: u32) -> u32 {
    unsafe {
        const STREAM: u32 = 0x04;
        const MODE: u32 = 0x08;
        const FLAGS: u32 = 0x0c;
        const BUF_CAP: u32 = 0x10;
        const CURSOR: u32 = 0x14;
        const PUSHBACK: u32 = 0x18;
        const EMPTY: u32 = 0x1c;
        const INDENT: u32 = 0x21c;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(this + STREAM, stream);
        wr32(this + MODE, 1);
        wr32(this + FLAGS, flags);
        wr32(this + BUF_CAP, 0x20);
        wr32(this + PUSHBACK, 0);
        ((this + EMPTY) as *mut u8).write_unaligned(0);
        wr32(this + INDENT, 0);
        wr32(this + CURSOR, 2);
        0
    }
});
