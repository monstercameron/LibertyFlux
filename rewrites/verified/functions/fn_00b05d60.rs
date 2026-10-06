// original: 0x00b05d60 update_state_byte_144
/// Refresh the state byte at `+0x144` from two helpers.
///
/// thiscall `(this)`: calls helper 1 (thiscall, no stack args) whose
/// answer becomes the first argument of helper 2 (cdecl, `(answer, 0)`),
/// stores helper 2's low answer byte at `this+0x144`, and returns helper
/// 2's full answer.
export!(thiscall, rw_00b05d60(this: u32) -> u32 {
    const STATE_OFF: u32 = 0x144;
    let a: u32 = callee_thiscall!(1, u32, this);
    let b: u32 = callee_cdecl!(2, u32, a, 0);
    unsafe {
        ((this + STATE_OFF) as *mut u8).write(b as u8);
    }
    b
});
