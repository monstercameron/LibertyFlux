// original: 0x00E39C90 audio_clear_entity_slot_flags

/// Release each non-null slot pointer through the shared audio release helper, in slot order. The twelve stdcall arguments are independent pointers; null entries are skipped and each non-null entry is passed in ECX with a zero stack option. The wrapper returns no meaningful value.
lf_checker_rt::export!(stdcall, rw_00e39c90(slot_0: u32, slot_1: u32, slot_2: u32, slot_3: u32, slot_4: u32, slot_5: u32, slot_6: u32, slot_7: u32, slot_8: u32, slot_9: u32, slot_10: u32, slot_11: u32) -> u32 {
    if slot_0 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_0, 0); }
    if slot_1 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_1, 0); }
    if slot_2 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_2, 0); }
    if slot_3 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_3, 0); }
    if slot_4 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_4, 0); }
    if slot_5 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_5, 0); }
    if slot_6 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_6, 0); }
    if slot_7 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_7, 0); }
    if slot_8 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_8, 0); }
    if slot_9 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_9, 0); }
    if slot_10 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_10, 0); }
    if slot_11 != 0 { let _release_result = lf_checker_rt::callee_thiscall!(1, u32, slot_11, 0); }
    0
});
