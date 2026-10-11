// original: 0x0053B4D0 leaderboard_info::vf2_ranked_coop_swat_assault_time

/// Call the object's second virtual method, which reports a row value. If that value equals `expected_value` and `output_slot` is non-null, store this template instance's relocated descriptor pointer and return `output_slot`; otherwise return null. The receiver is passed in ECX and the two explicit arguments are stack words.
lf_checker_rt::export!(thiscall, rw_0053b4d0(this: u32, output_slot: u32, expected_value: u32) -> u32 {
    unsafe {
        const DESCRIPTOR_VA: u32 = 0x00FDFD84;
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable as *const u32).wrapping_add(1).read_unaligned();
        let report: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        if report(this) != expected_value || output_slot == 0 { return 0; }
        let descriptor = lf_checker_rt::relocated(DESCRIPTOR_VA.wrapping_add(0));
        (output_slot as *mut u32).write_unaligned(descriptor);
        output_slot
    }
});
