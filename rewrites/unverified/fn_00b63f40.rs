// original: 0x00B63F40 veh_clamp_store_60
/// Clamp an argument to 0x61A8 (signed) and store its low word at `[this+0x60]`.
///
/// `v = min(a0, 0x61A8)` with a signed 32-bit comparison, then the low 16 bits
/// of `v` are stored to `[this+0x60]` (upper half of the dword cleared) and
/// returned. Thiscall, one stack word; entry registers except ECX are ignored.
export!(thiscall, rw_00b63f40(this: u32, a0: u32) -> u32 {
    unsafe {
        const LIMIT: i32 = 0x61A8;
        const SLOT: u32 = 0x60;
        let v = if (a0 as i32) > LIMIT { LIMIT } else { a0 as i32 };
        let out = (v as u32) & 0xFFFF;
        ((this + SLOT) as *mut u32).write_unaligned(out);
        out
    }
});
