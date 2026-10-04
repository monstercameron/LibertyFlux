// original: 0x00900d10 zero_head_init_attach
/// Zero the head word, run the init helper, then attach the shared global.
///
/// Forwards the argument to the init callee (thiscall/1, stubbed by the
/// checker), discards its answer and stores the shared dword next to the
/// zeroed head. Returns `this`.
export!(thiscall, rw_00900d10(this: u32, a: u32) -> u32 {
    unsafe {
        (this as *mut u32).write(0);
        callee_thiscall!(1, u32, this, a);
        let g = *global::<u32>(0x118f4e0);
        (this as *mut u32).add(1).write(g);
        this
    }
});
