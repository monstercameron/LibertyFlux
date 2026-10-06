// original: 0x00DDE0F0 UITextField virtual forward
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Forward `key` to the primary child's virtual slot `+0x214` and return
/// `key` itself (the callee's answer is ignored).
/// Original: thiscall, one stack word, result in eax.
lf_checker_rt::export!(thiscall, rw_00DDE0F0(this: u32, key: u32) -> u32 {
    unsafe {
        const CHILD: u32 = 0x1E8;
        const SLOT: u32 = 0x214;
        let child = ((this + CHILD) as *const u32).read_unaligned();
        let vt = ((child) as *const u32).read_unaligned();
        let slot = ((vt + SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            unsafe { core::mem::transmute(slot as usize) };
        f(child, key);
        key
    }
});
