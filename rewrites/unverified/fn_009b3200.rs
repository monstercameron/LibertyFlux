// original: 0x009B3200 audPoliceScanner::report_player_spotted

/// Reads the player's three position words, preserves their exact bit
/// patterns in a temporary vector, and sends that vector with four fixed
/// flags to the scanner report helper. The helper's stack pointer argument
/// is compared through a call-time snapshot of the three words.
lf_checker_rt::export!(thiscall, rw_009b3200(this: u32, _event: u32) -> u32 {
    unsafe {
        let context = lf_checker_rt::callee_cdecl!(1, u32, 0);
        let ped = ((context + 0x20) as *const u32).read_unaligned();
        let mut position = [
            ((ped + 0x30) as *const u32).read_unaligned(),
            ((ped + 0x34) as *const u32).read_unaligned(),
            ((ped + 0x38) as *const u32).read_unaligned(),
        ];
        let position_pointer = position.as_mut_ptr() as u32;
        let _ = lf_checker_rt::callee_thiscall!(2, u32, this, position_pointer, 0, 1, 1, 1);
        0
    }
});
