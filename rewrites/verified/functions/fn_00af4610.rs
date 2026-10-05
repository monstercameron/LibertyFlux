// original: 0x00AF4610 drawable_attach_set (proposed)

/// Attach a slot set to the drawable and bind each live entry.
///
/// Runs the open callee over (this, `arg`), stamps the drawable vtable
/// pointer at `this+0`, runs the attach callee with (`arg`, this+0x80),
/// then walks the 16-bit entry count at this+0x84: for each entry whose
/// base-plus-offset sum is nonzero the bind callee runs with (sum, `arg`),
/// entries being 0x6C bytes apart. Returns the object pointer.
///
/// Original: 0x00AF4610 (thiscall, one stack word, three direct callees).
lf_checker_rt::export!(thiscall, rw_00af4610(this: u32, arg: u32) -> u32 {
    unsafe {
        const OPEN_CALLEE: u32 = 1;
        const ATTACH_CALLEE: u32 = 2;
        const BIND_CALLEE: u32 = 3;
        const VTABLE: u32 = 0x00EA839C;
        const SET_OFF: u32 = 0x80;
        const COUNT_OFF: u32 = 0x84;
        const ENTRY_STRIDE: u32 = 0x6C;
        lf_checker_rt::callee_thiscall!(OPEN_CALLEE, u32, this, arg);
        let set = this.wrapping_add(SET_OFF);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        lf_checker_rt::callee_thiscall!(ATTACH_CALLEE, u32, arg, set);
        let count = ((set.wrapping_add(4)) as *const u16).read_unaligned() as u32;
        let mut i: u32 = 0;
        let mut off: u32 = 0;
        while i < count {
            let sum = ((set) as *const u32).read_unaligned().wrapping_add(off);
            if sum != 0 {
                lf_checker_rt::callee_thiscall!(BIND_CALLEE, u32, sum, arg);
            }
            i = i.wrapping_add(1);
            off = off.wrapping_add(ENTRY_STRIDE);
        }
        this
    }
});
