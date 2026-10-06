// original: 0x00b063f0 load_or_reset_state
/// Load a state record, or reset when the count is not positive.
///
/// thiscall `(this, src, b, c, d, count)`: stores `count` at `+0x184`,
/// runs two init helpers and the float-field setter (helper 3, thiscall
/// with -1.0f). When `count > 0` (SIGNED) it then stores the global tick
/// at `+0x180`, copies four words from `src` to `+0x160` in the
/// original's exact order (words 0-2 read, then written, then word 3
/// read and written, so overlapping `src` aliases identically), and
/// stores `b`/`c`/`d` at `+0x188`/`+0x18c`/`+0x190`, returning `d`.
/// Otherwise it calls the full reset (helper 4) and returns its answer.
export!(thiscall, rw_00b063f0(this: u32, src: u32, b: u32, c: u32, d: u32, count: u32) -> u32 {
    const TICK: u32 = 0x0117_35C4;
    const NEG_ONE_BITS: u32 = 0xBF80_0000;
    unsafe {
        ((this + 0x184) as *mut u32).write_unaligned(count);
    }
    let _: u32 = callee_thiscall!(1, u32, this);
    let _: u32 = callee_thiscall!(2, u32, this);
    let _: u32 = callee_thiscall!(3, u32, this, NEG_ONE_BITS);
    if (count as i32) > 0 {
        unsafe {
            let tick = *global::<u32>(TICK);
            ((this + 0x180) as *mut u32).write_unaligned(tick);
            let w0 = (src as *const u32).read_unaligned();
            let w1 = ((src + 4) as *const u32).read_unaligned();
            let w2 = ((src + 8) as *const u32).read_unaligned();
            ((this + 0x160) as *mut u32).write_unaligned(w0);
            ((this + 0x164) as *mut u32).write_unaligned(w1);
            ((this + 0x168) as *mut u32).write_unaligned(w2);
            let w3 = ((src + 12) as *const u32).read_unaligned();
            ((this + 0x16C) as *mut u32).write_unaligned(w3);
            ((this + 0x188) as *mut u32).write_unaligned(b);
            ((this + 0x18C) as *mut u32).write_unaligned(c);
            ((this + 0x190) as *mut u32).write_unaligned(d);
        }
        d
    } else {
        let r: u32 = callee_thiscall!(4, u32, this);
        r
    }
});
