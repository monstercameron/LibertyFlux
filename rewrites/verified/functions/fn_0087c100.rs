// original: 0x0087c100 crmt_node_alloc_init (proposed)
/// Allocate and initialise a node for an owner.
///
/// Reuses the spare block registered at `a+0x20` through intercepted callee
/// 1 (thiscall/1, word 5) when one is registered and the callee answers
/// non-null, otherwise allocates 0x34 bytes through the TLS heap (slot 0,
/// allocator object at `[slot]+8`, slot 8 of its table with size 0x34,
/// flags 0x10 and a null tag) and runs the base initializer (intercepted
/// callee 3, thiscall/1, word 5, ecx-preserving) over it. The fresh block
/// gets the blend vtable (file VA 0x00FE8544, relocated), a cleared flag
/// byte at `+0x30`, the owner link at `+0x08` and word 5 at `+0x06`. When the
/// heap answers null the original falls through and faults writing to
/// address 8; the rewrite faults the same way, and the contract checks the
/// fault parity. Returns the block. All comparisons are null checks.
///
/// Original: cdecl/1, two direct plus one indirect call, no floats.
export!(cdecl, rw_0087c100(a: u32) -> u32 {
    /// Spare-block link in the owner.
    const SPARE_OFF: u32 = 0x20;
    /// Blend-node vtable (file VA; relocated at load).
    const BLEND_VTABLE: u32 = 0x00FE8544;
    /// Fabricated TLS slot holding the heap anchor.
    const TLS_SLOT: usize = 0;
    /// Allocator slot in the heap object's table.
    const ALLOC_SLOT: u32 = 8;
    /// Word passed to the initializer calls.
    const INIT_WORD: u32 = 5;
    unsafe {
        let spare = ((a + SPARE_OFF) as *const u32).read_unaligned();
        if spare != 0 {
            let r = callee_thiscall!(1, u32, spare, INIT_WORD);
            if r != 0 {
                ((r + 0x08) as *mut u32).write_unaligned(a);
                ((r + 0x06) as *mut u16).write_unaligned(INIT_WORD as u16);
                return r;
            }
        }
        let tls = tls_slot(TLS_SLOT);
        let alloc = ((tls + 8) as *const u32).read_unaligned();
        let vt = (alloc as *const u32).read_unaligned();
        let tgt = ((vt + ALLOC_SLOT) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let r = f(alloc, 0x34, 0x10, 0);
        if r == 0 {
            // Original falls through with a null block and faults here.
            (8u32 as *mut u32).write_unaligned(a);
            ((0u32 + 0x06) as *mut u16).write_unaligned(INIT_WORD as u16);
            return 0;
        }
        callee_thiscall!(3, u32, r, INIT_WORD);
        (r as *mut u32).write_unaligned(relocated(BLEND_VTABLE));
        ((r + 0x30) as *mut u8).write(0);
        ((r + 0x08) as *mut u32).write_unaligned(a);
        ((r + 0x06) as *mut u16).write_unaligned(INIT_WORD as u16);
        r
    }
});
