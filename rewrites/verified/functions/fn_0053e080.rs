// original: 0x0053E080 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_TeamVip_BG,player_schema::LeaderboardInfo,10>::LeaderboardInfo,10>_2

/// Looks up a type value through the object's second virtual slot.
/// `this` points to a vtable pointer at byte offset 0; the lookup function is at vtable offset 4 and receives `this` in ECX.
/// The two stack arguments are a writable output word and the expected type value. The lookup runs before either comparison branch.
/// When the lookup equals the expected value and the output pointer is non-null, the function writes the relocated schema token and returns the output pointer. Otherwise it returns zero.
/// The token is an address-sized data value, so it is derived with `relocated` from the preferred image address.
lf_checker_rt::export!(thiscall, rw_fn_0053e080(this: u32, output_word: u32, expected_type: u32) -> u32 {
    unsafe {
                const OBJECT_VTABLE_OFFSET: u32 = 0;
                const KEY_LOOKUP_SLOT_OFFSET: u32 = 4;
                const SCHEMA_TOKEN_FILE_VA: u32 = 0x00fd2b2c;

                let vtable = (this.wrapping_add(OBJECT_VTABLE_OFFSET) as *const u32).read_unaligned();
                let lookup_address = vtable.wrapping_add(KEY_LOOKUP_SLOT_OFFSET);
                let lookup_target = (lookup_address as *const u32).read_unaligned();
                let lookup: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(lookup_target as usize);
                let actual_type = lookup(this);
                if actual_type != expected_type {
                    return 0;
                }
                if output_word == 0 {
                    return 0;
                }
                let token = lf_checker_rt::relocated(SCHEMA_TOKEN_FILE_VA);
                (output_word as *mut u32).write_unaligned(token);
                output_word
    }
});
