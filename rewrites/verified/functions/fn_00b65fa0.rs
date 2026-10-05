// original: 0x00B65FA0 veh_clamp_store_04
/// Clamp the argument's low word to 0x61A8 (unsigned) and store it at `[this+4]`.
///
/// `v = min(a0 & 0xFFFF, 0x61A8)` with an unsigned 16-bit comparison; `v` is
/// stored to `[this+4]` and returned. Thiscall, one stack word; entry registers
/// except ECX are ignored.
export!(thiscall, rw_00b65fa0(this: u32, a0: u32) -> u32 {
    unsafe {
        const LIMIT: u32 = 0x61A8;
        const SLOT: u32 = 4;
        let a = a0 & 0xFFFF;
        let v = if a > LIMIT { LIMIT } else { a };
        ((this + SLOT) as *mut u32).write_unaligned(v);
        v
    }
});
