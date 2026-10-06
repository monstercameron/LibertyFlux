// original: 0x00d69640 checked_step_prev
/// Guard that forwards to a worker routine when the object's link word is set.
///
/// `this` points to an object whose word at offset 4 is a link. The word is
/// loaded; if it is nonzero the function tail-chains to routine 1 with the
/// loaded word as `this`, returning routine 1's answer. If the word is zero
/// it returns a zero low byte without calling. On the not-taken path the
/// original keeps the high bytes of entry EAX, so the return register is
/// not compared.
export!(thiscall, rw_00d69640(this_: u32) -> u32 {
    unsafe {
        let v = ((this_.wrapping_add(4)) as *const u32).read();
        if v != 0 {
            callee_thiscall!(1, u32, v)
        } else {
            0
        }
    }
});
