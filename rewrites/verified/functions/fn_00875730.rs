// original: 0x00875730 rage::crmtRequestFilter::vf0

/// Scalar-deleting destructor: run the object teardown, conditionally free
/// `this` through the thread memory manager, and return `this`.
///
/// Calls the teardown helper (0x008757d0, thiscall, no stack arguments) with
/// `this` in ECX, then reads the low bit of the `flags` stack word. When the
/// bit is set and `this` is non-null, loads the thread-local manager object
/// (TLS slot 0, manager pointer at `+8`), reads its vtable, and calls the
/// free routine at vtable slot `+0x0c` (thiscall: ECX is the manager, one
/// stack word is `this`). Returns `this` unchanged, including null.
///
/// Original: 0x00875730 (thiscall, one stack word; the callee pops 4 bytes).
/// Edge cases: null `this` skips the free call and returns 0; a clear low
/// flag bit skips it and returns `this`.
lf_checker_rt::export!(thiscall, rw_00875730(this: u32, flags: u32) -> u32 {
    const TEARDOWN: u32 = 1;
    const FREE_SLOT: u32 = 0x0c;
    const MANAGER_OFF: u32 = 8;
    unsafe {
        lf_checker_rt::callee_thiscall!(TEARDOWN, u32, this);
        if flags & 1 != 0 && this != 0 {
            let tls0 = lf_checker_rt::tls_slot(0);
            let manager = ((tls0 + MANAGER_OFF) as *const u32).read_unaligned();
            let vtable = (manager as *const u32).read_unaligned();
            let target = ((vtable as *const u8).add(FREE_SLOT as usize) as *const u32)
                .read_unaligned();
            let free: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(target as usize);
            free(manager, this);
        }
    }
    this
});
