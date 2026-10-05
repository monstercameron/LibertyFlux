// original: 0x00a8a350 pool_select_slot_b (proposed)

/// Select a slot offset from a nine-way switch, then forward to the slot
/// setter.
///
/// `this` is the pool object, `which` selects the added offset (0 and 4 add
/// 9, 1 and 7 (an instruction of the original)) and
/// `base` is the slot base. A `which` above 8 returns `base` unchanged and
/// makes no call.
///
/// Original: 0x00A8A350 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00a8a350(this: u32, which: u32, base: u32) -> u32 {
    unsafe {
        const CALLEE_SET_SLOT: u32 = 1;
        const OFFSETS: [u32; 9] = [9, 12, 15, 18, 9, 21, 24, 12, 15];
        if which > 8 {
            return base;
        }
        lf_checker_rt::callee_thiscall!(
            CALLEE_SET_SLOT,
            u32,
            this,
            base.wrapping_add(OFFSETS[which as usize])
        )
    }
});
