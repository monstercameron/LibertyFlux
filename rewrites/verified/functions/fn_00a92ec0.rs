// original: 0x00a92ec0 stream_slot_select

/// Selects a slot from a scaled key and bumps the use count.
///
/// Computes `(key >> 8) * [this+0xC] + [this]` (arithmetic shift, wrapping),
/// notifies the callee (callee 1, thiscall) with the raw key, increments
/// the counter at `this+0x14` and returns the slot. One call, no globals.
/// Original: 0x00A92EC0 (thiscall, ECX + one stack word), 35 bytes.
lf_checker_rt::export!(thiscall, rw_00a92ec0(this: u32, key: u32) -> u32 {
    unsafe {
        const BASE_OFF: u32 = 0x00;
        const MULT_OFF: u32 = 0x0C;
        const COUNT_OFF: u32 = 0x14;
        const NOTIFY: u32 = 1;
        let mult = (this.wrapping_add(MULT_OFF) as *const i32).read_unaligned();
        let base = (this.wrapping_add(BASE_OFF) as *const i32).read_unaligned();
        let scaled = (key as i32).wrapping_shr(8);
        let slot = scaled.wrapping_mul(mult).wrapping_add(base);
        let _: u32 = lf_checker_rt::callee_thiscall!(NOTIFY, u32, this, key);
        let count = (this.wrapping_add(COUNT_OFF) as *const u32).read_unaligned();
        (this.wrapping_add(COUNT_OFF) as *mut u32).write_unaligned(count.wrapping_add(1));
        slot as u32
    }
});
