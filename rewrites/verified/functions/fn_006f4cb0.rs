// original: 0x006f4cb0 accum_pair_sum
/// Returns the folded pair value for slot 4 plus the slot-2 contribution.
///
/// Slot 4 is always folded: it is sign-extended to 64 bits and run twice
/// through the shared 64-bit pair helpers. While slot 4 is clear, slot 2 is
/// folded the same way and added; otherwise the raw slot 2 feeds the sum
/// directly. The combined value is returned.
export!(thiscall, rw_006f4cb0(this: u32) -> u32 {
    unsafe {
        let obj = this as *const u32;
        let first = if *obj.add(4) == 0 {
            let ext = *obj.add(2) as i32 as i64;
            let a = callee_stdcall!(
                1, u32, *obj.add(7), 0, ext as u32, (ext >> 32) as u32
            );
            // The stub answers a 64-bit value with high word 0 (scripted).
            callee_stdcall!(2, u32, a, 0, 100, 0)
        } else {
            *obj.add(2)
        };
        let ext = *obj.add(4) as i32 as i64;
        let a = callee_stdcall!(
            1, u32, *obj.add(7), 0, ext as u32, (ext >> 32) as u32
        );
        callee_stdcall!(2, u32, a, 0, 100, 0).wrapping_add(first)
    }
});
