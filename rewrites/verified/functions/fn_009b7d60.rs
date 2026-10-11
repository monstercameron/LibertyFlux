// original: 0x009B7D60 fn_009b7d60
/// For each of two 32-bit float fields, compares it with the threshold word and, when unequal or unordered, calls a conversion helper followed by the instruction dispatcher and writes the reset sentinel. It clears the camera reset byte afterward. Float equality follows IEEE single-precision rules, including equality of signed zeros; no arithmetic result is produced.
lf_checker_rt::export!(thiscall, rw_009b7d60(this: u32) -> u32 {
    const FIRST_LIMIT: u32 = 0x538;
    const SECOND_LIMIT: u32 = 0x53C;
    const RESET_VALUE: u32 = 0xC7C34F80;
    const THRESHOLD_VA: u32 = 0x00E833E8;
    const RESET_FLAG_VA: u32 = 0x0128E3F9;
    unsafe fn read_word(address: u32) -> u32 {
        (address as *const u32).read_unaligned()
    }
    unsafe fn write_word(address: u32, value: u32) {
        (address as *mut u32).write_unaligned(value)
    }
    unsafe fn differs_or_unordered(value: u32, threshold: u32) -> bool {
        let value = f32::from_bits(value);
        let threshold = f32::from_bits(threshold);
        value != threshold
    }
    unsafe fn process(this: u32, field: u32, helper: u32, update: u32) {
        let value = read_word(this.wrapping_add(field));
        let threshold = read_word(lf_checker_rt::relocated(THRESHOLD_VA));
        if differs_or_unordered(value, threshold) {
            let converted = if helper == 1 {
                lf_checker_rt::callee_stdcall!(1, u32, value)
            } else {
                lf_checker_rt::callee_stdcall!(3, u32, value)
            };
            let _ = lf_checker_rt::callee_thiscall!(2, u32, this, converted);
            write_word(this.wrapping_add(update), RESET_VALUE);
        }
    }
    unsafe {
        process(this, FIRST_LIMIT, 1, FIRST_LIMIT);
        process(this, SECOND_LIMIT, 3, SECOND_LIMIT);
        (lf_checker_rt::global::<u8>(RESET_FLAG_VA)).write(0);
    }
    0
});
