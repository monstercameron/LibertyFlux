// original: 0x00e3f480 StatsValue_Quantize
// 0x00E3F480: quantize a value down to a whole number of record steps.
// (thiscall/1)
export!(thiscall, rw_00e3f480(this: *const u8, value: u32) -> u32 {
    unsafe {
        let step = *(this.add(0x310));
        if value == 0 || step == 0 {
            return 0;
        }
        let units = (value as i32) / (step as i32);
        ((step as i32).wrapping_mul(units)) as u32
    }
});
