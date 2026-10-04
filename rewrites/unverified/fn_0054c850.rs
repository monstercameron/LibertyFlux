// original: 0x0054c850 rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Episodic_BG_22, player_schema::LeaderboardInfo, 10>::vf2

/// Bind this board's row handle: record the row magic when the row id matches.
///
/// The board object (`this`) is asked for its current row id through slot 1
/// of its own function table (`SLOT_ONE` words in; callee id 1, thiscall, no
/// stack args, `this` still in `ecx`). When the answer equals `wanted` and
/// `out` is non-null, the row descriptor address is stored at `out` and
/// `out` is returned. A mismatching id or a null `out` gives null without
/// storing anything.
///
/// The stored word is an image pointer (file VA `ROW_DESC_VA`, relocated at
/// runtime through the checker helper: the worker maps the image at its own
/// base, so the file value is never stored directly).
///
/// Original: thiscall (`this` in `ecx`, two stack words). The indirect call
/// is intercepted by planting the stub address in the fabricated table, so
/// both sides call the same stub.
lf_checker_rt::export!(thiscall, rw_0054c850(this: u32, out: u32, wanted: u32) -> u32 {
    unsafe {
        const SLOT_ONE: u32 = 4;
        const ROW_DESC_VA: u32 = 0xfd0b64;
        let vtable = (this as *const u32).read_unaligned();
        let slot = (vtable.wrapping_add(SLOT_ONE) as *const u32).read_unaligned();
        let current: u32 = {
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
            f(this)
        };
        if current != wanted {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        (out as *mut u32).write_unaligned(lf_checker_rt::relocated(ROW_DESC_VA));
        out
    }
});
