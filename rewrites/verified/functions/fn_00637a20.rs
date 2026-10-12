// original: 0x00637A20 set_effect_instance_enabled

/// Set an effect instance's enabled byte. When `this + 0x78` points to an
/// instance counter, a transition to enabled increments its word at `+0x114`;
/// a transition to disabled decrements that word only when it is nonzero.
/// If no counter object is present, only the byte at `this + 0x1D0` is set.
/// The object is passed in ECX and the requested byte is the single stack
/// argument; the callee removes that argument.
lf_checker_rt::export!(thiscall, rw_00637A20(this: u32, enabled: u32) -> u32 {
    unsafe {
        const COUNTER_POINTER: u32 = 0x78;
        const ENABLED_BYTE: u32 = 0x1D0;
        const ACTIVE_COUNT: u32 = 0x114;

        let requested = enabled as u8;
        let counter = (this.wrapping_add(COUNTER_POINTER) as *const u32).read_unaligned();
        let state_address = this.wrapping_add(ENABLED_BYTE) as *mut u8;
        if counter == 0 {
            state_address.write(requested);
            return 0;
        }

        let previous = state_address.read();
        if requested != previous {
            let count_address = counter.wrapping_add(ACTIVE_COUNT) as *mut u32;
            if requested != 0 {
                let count = count_address.read_unaligned();
                count_address.write_unaligned(count.wrapping_add(1));
            } else {
                let count = count_address.read_unaligned();
                if count != 0 {
                    count_address.write_unaligned(count - 1);
                }
            }
        }
        state_address.write(requested);
        0
    }
});
