// original: 0x00a131f0 dist_pair_and_flag_store (proposed)
/// Store two distance values and mark them valid.
///
/// Copies the bit patterns of `d0` and `d1` to `this + 0x2f0` and
/// `this + 0x2f4` and sets the byte at `this + 0x2f8` to 1. The values
/// move as bits (SSE `movss`), so NaN payloads are preserved. No return
/// value is set. Thiscall, two stack arguments.
export!(thiscall, rw_00a131f0(this: u32, d0: u32, d1: u32) -> u32 {
    unsafe {
        const D0_OFF: u32 = 0x2f0;
        const D1_OFF: u32 = 0x2f4;
        const FLAG_OFF: u32 = 0x2f8;
        ((this + D0_OFF) as *mut u32).write_unaligned(d0);
        ((this + FLAG_OFF) as *mut u8).write(1);
        ((this + D1_OFF) as *mut u32).write_unaligned(d1);
        0
    }
});
