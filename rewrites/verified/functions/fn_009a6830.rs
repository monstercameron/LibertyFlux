// original: 0x009A6830 audio_apply_slot_a (proposed)

/// Resolves a slot id through two helpers and applies it to channel A.
///
/// thiscall, one stack word `arg`: calls the info getter (callee 1) on
/// `this`, feeds dword `INFO_SLOT` (+4) of the result into the id mapper
/// (callee 2), then calls the channel-A applier (callee 3, the function at
/// 0x009A6860) with (`arg`, mapped id). Returns the applier's answer.
lf_checker_rt::export!(thiscall, rw_009a6830(this: u32, arg: u32) -> u32 {
    unsafe {
        const INFO_GETTER: u32 = 1;
        const ID_MAPPER: u32 = 2;
        const APPLY_A: u32 = 3;
        const INFO_SLOT: u32 = 4;
        let info: u32 = lf_checker_rt::callee_thiscall!(INFO_GETTER, u32, this);
        let slot = (info.wrapping_add(INFO_SLOT) as *const u32).read_unaligned();
        let mapped: u32 = lf_checker_rt::callee_thiscall!(ID_MAPPER, u32, this, slot);
        lf_checker_rt::callee_thiscall!(APPLY_A, u32, this, arg, mapped)
    }
});
