// original: 0x00538940 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_CompMafiaWork, player_schema::LeaderboardInfo, 10>::vf2
/// Validate a candidate leaderboard entry through the object's second vtable slot.
///
/// The object begins with a vtable pointer, and its second slot is a
/// parameterless thiscall predicate. This wrapper compares that method's
/// return with the expected 32-bit value. If they match and the output-word
/// pointer is non-null, it writes a relocated image-data token and returns
/// the pointer; otherwise it returns zero.
lf_checker_rt::export!(thiscall, rw_00538940(this: u32, output_word: u32, expected: u32) -> u32 {
    unsafe {
        const VTABLE_POINTER_OFFSET: u32 = 0;
        const VALIDATOR_SLOT_OFFSET: u32 = 4;
        const TOKEN_FILE_VA: u32 = 0x00FCEAB4;
        #[inline(always)]
        unsafe fn read_word(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_word(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        let vtable = unsafe { read_word(this.wrapping_add(VTABLE_POINTER_OFFSET)) };
        let validator_address = unsafe {
            read_word(vtable.wrapping_add(VALIDATOR_SLOT_OFFSET))
        };
        let validator: extern "thiscall" fn(u32) -> u32 = unsafe {
            core::mem::transmute(validator_address as usize)
        };
        let actual = validator(this);
        if actual != expected || output_word == 0 {
            return 0;
        }
        unsafe { write_word(output_word, lf_checker_rt::relocated(TOKEN_FILE_VA)); }
        output_word
    }
});
