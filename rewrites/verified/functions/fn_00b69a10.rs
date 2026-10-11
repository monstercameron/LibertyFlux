// original: 0x00B69A10 heli_part_slot_1_broken

/// Read the signed component selector byte at `this + 0x1ef1`. Negative
/// selectors and a null component table return false; otherwise call the
/// component helper with the table in ECX and the selector sign-extended on
/// the stack, returning whether its AL result is nonzero. This thiscall
/// wrapper returns its boolean in AL.
lf_checker_rt::export!(thiscall, rw_00B69A10(this: u32) -> u32 {
    unsafe {
        const COMPONENT_TABLE: u32 = 0xdc4;
        const SELECTOR: u32 = 0x1ef1;
        const COMPONENT_HELPER: u32 = 1;
        let table = (this.wrapping_add(COMPONENT_TABLE) as *const u32).read_unaligned();
        if table == 0 { return 0; }
        let raw_selector = (this.wrapping_add(SELECTOR) as *const u8).read_unaligned();
        let signed_selector = raw_selector as i8;
        if signed_selector < 0 { return 0; }
        let answer = lf_checker_rt::callee_thiscall!(
            COMPONENT_HELPER, u32, table, signed_selector as i32 as u32
        );
        u32::from((answer as u8) != 0)
    }
});
