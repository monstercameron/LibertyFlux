// original: 0x008de110 alloc_then_dispatch (proposed)

/// Allocate an aligned block, then dispatch it through its virtual slot.
///
/// Aligns `base` up to 16 bytes and allocates `size` through the arena
/// (callee 1, called with the arena singleton). A null answer is returned
/// as-is; otherwise virtual slot `+0x10` of the block (callee 2) runs with
/// the block as its object, and its answer is returned. The original
/// tail-jumps to that slot; the rewrite calls it and forwards the result,
/// which is stack-neutral. Cdecl, two stack arguments.
lf_checker_rt::export!(cdecl, rw_008de110(base: u32, size: u32) -> u32 {
    unsafe {
        const ARENA: u32 = 0x0117_5c58;
        const ALIGN_MASK: u32 = 0x0f;
        const DISPATCH_SLOT: u32 = 0x10;
        const CALLEE_ALLOC: u32 = 1;
        let aligned = base.wrapping_add(base.wrapping_neg() & ALIGN_MASK);
        let block = lf_checker_rt::callee_thiscall!(
            CALLEE_ALLOC,
            u32,
            lf_checker_rt::relocated(ARENA),
            aligned,
            size
        );
        if block == 0 {
            return 0;
        }
        let vtable = (block as *const u32).read_unaligned();
        let dispatch = ((vtable + DISPATCH_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(dispatch as usize);
        f(block)
    }
});
