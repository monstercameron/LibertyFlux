// original: 0x00DDEB90 UITextField reset and measure
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Reset the field updater with (`key`, 0), then measure the text length
/// through the primary child's virtual slot `+0x20C` (scan for the NUL),
/// store the length's low byte at the cursor `+0x208`, and return the length.
/// Original: thiscall, one stack word, length in eax.
lf_checker_rt::export!(thiscall, rw_00DDEB90(this: u32, key: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 0x1E8;
        const CURSOR: u32 = 0x208;
        const SLOT: u32 = 0x20C;
        const RESET: u32 = 1;
        lf_checker_rt::callee_thiscall!(RESET, u32, this, key, 0);
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let vt = ((child) as *const u32).read_unaligned();
        let slot = ((vt + SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        let s = f(child);
        let mut p = s;
        while ((p) as *const u8).read() != 0 {
            p = p.wrapping_add(1);
        }
        let len = p.wrapping_sub(s);
        ((this + CURSOR) as *mut u8).write(len as u8);
        len
    }
});
