// original: 0x0062BD10 tls_alloc_and_tail_init (proposed)

/// Allocate a 0x60-byte block through the thread-local allocator, then tail
/// off to the initializer.
///
/// Same allocator call as the `vf5` family (TLS slot 0, slot `+8`, size
/// `0x60`). On success the block address is passed in the object register to
/// the initializer the original tail-jumps to (patched tail callee), whose
/// result is the result; on allocation failure returns null (cdecl).
lf_checker_rt::export!(cdecl, rw_0062bd10() -> u32 {
    unsafe {
        const ALLOC_SIZE: u32 = 0x60;
        const CALLEE_INIT: u32 = 2;
        let holder = lf_checker_rt::tls_slot(0);
        let obj = ((holder + 8) as *const u32).read_unaligned();
        let vtable = (obj as *const u32).read_unaligned();
        let tgt = ((vtable + 8) as *const u32).read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let mem = alloc(obj, ALLOC_SIZE, 0x10, 0);
        if mem == 0 {
            0
        } else {
            lf_checker_rt::callee_thiscall!(CALLEE_INIT, u32, mem)
        }
    }
});
