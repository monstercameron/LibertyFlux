// original: 0x00e3da90 StatsTick_IsOverLimit
// 0x00E3DA90: report whether the global tick count (as a float) is above
// the record's limit. (thiscall/0, AL-clean: upper EAX is zero)
export!(thiscall, rw_00e3da90(this: *const u8) -> u32 {
    unsafe {
        let ticks = *global::<u32>(0x1173594);
        // Unsigned 32-bit to double, the way the original builds it:
        // signed convert plus 2^32 when the sign bit is set.
        let mut wide = (ticks as i32) as f64;
        if (ticks >> 31) != 0 {
            wide += 4294967296.0;
        }
        let level = wide as f32;
        let limit = *(this.add(0x18) as *const f32);
        (level > limit) as u32
    }
});
