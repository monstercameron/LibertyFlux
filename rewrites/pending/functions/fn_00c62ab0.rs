// original: 0x00c62ab0 anim_register_default
/// Default registration: registers the default hook with zero weights.
///
/// Forwards this player with zero weights and the default hook address to the
/// registrar, ignoring its own stack argument, and returns the answer.
export!(thiscall, rw_00c62ab0(this: u32, _ignored: u32) -> u32 {
    callee_thiscall!(0, u32, this, 0, relocated(0x4016A0), 0)
});
