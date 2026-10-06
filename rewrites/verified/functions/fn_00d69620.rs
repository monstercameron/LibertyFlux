// original: 0x00d69620 checked_activate
/// Guard that forwards to a worker routine when the object's link word is set.
///
/// `this` points to an object whose word at offset 4 is a link. The word is
/// loaded; if it is nonzero the function tail-chains to routine 1 with the
/// loaded word as `this` and the caller's stack word forwarded unchanged,
/// returning routine 1's answer. If the word is zero it returns without
/// calling, popping the stack word. On the not-taken path the original
/// leaves entry EAX in place, so the return register is not compared.
export!(thiscall, rw_00d69620(this_: u32, arg: u32) -> u32 {
    unsafe {
        let v = ((this_.wrapping_add(4)) as *const u32).read();
        if v != 0 {
            callee_thiscall!(1, u32, v, arg)
        } else {
            0
        }
    }
});
