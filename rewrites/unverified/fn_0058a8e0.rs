// original: 0x0058A8E0 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_Race_222, player_schema::LeaderboardInfo, 10>::vf2

/// Hand out this leaderboard's interface pointer when the id matches.
///
/// Calls the object's own id slot (second entry of the vtable at `[this]`,
/// intercepted by a planted stub) with `this` unchanged. When the returned id
/// equals `want` and `out` is non-null, stores this class's vtable pointer
/// (`0x00fda7ac` file VA, relocated at load) into `*out` and returns `out`;
/// otherwise returns 0 (a mismatched id, or a null `out`, yields 0).
///
/// Original: 0x0058A8E0 (thiscall: `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_0058A8E0(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        /// Vtable this object hands out (file VA; relocated at load).
        const IFACE_VTABLE: u32 = 0xfda7ac;
        /// Byte offset of the id slot within the object's vtable.
        const ID_SLOT: u32 = 4;
        let vtable = (this as *const u32).read_unaligned();
        let id_fn_addr = (vtable.wrapping_add(ID_SLOT) as *const u32).read_unaligned();
        let get_id: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(id_fn_addr as usize);
        let got = get_id(this);
        if got != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(IFACE_VTABLE));
        out
    }
});
