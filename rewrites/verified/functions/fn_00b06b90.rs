// original: 0x00b06b90 ratio_or_expired
/// Elapsed-tick ratio, or expired.
///
/// thiscall `(this, out)`: `elapsed = tick - start` where `tick` is the
/// global tick counter and `start` the object word at `+0x180`. Both
/// `elapsed` and the object word at `+0x184` (the limit) are compared as
/// SIGNED 32-bit. When `elapsed > limit` (signed) the function stores
/// nothing and returns `elapsed` with its low byte cleared; otherwise it
/// stores `(float)elapsed / (float)limit` (signed conversions, SSE
/// divide) at `out` and returns `out` with its low byte set to 1 (the
/// original reloads the return register with the out-pointer for the
/// store and only sets the low byte afterwards).
export!(thiscall, rw_00b06b90(this: u32, out: u32) -> u32 {
    unsafe {
        const START_OFF: u32 = 0x180;
        const LIMIT_OFF: u32 = 0x184;
        const TICK: u32 = 0x0117_35C4;
        let tick = *global::<u32>(TICK);
        let start = ((this + START_OFF) as *const u32).read_unaligned();
        let limit = ((this + LIMIT_OFF) as *const u32).read_unaligned();
        let elapsed = tick.wrapping_sub(start);
        if (elapsed as i32) > (limit as i32) {
            elapsed & 0xFFFF_FF00
        } else {
            let num = core::hint::black_box(elapsed as i32) as f32;
            let den = core::hint::black_box(limit as i32) as f32;
            let q = core::hint::black_box(num) / core::hint::black_box(den);
            ((out) as *mut u32).write_unaligned(q.to_bits());
            (out & 0xFFFF_FF00) | 1
        }
    }
});
