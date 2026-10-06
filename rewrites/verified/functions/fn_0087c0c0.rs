// original: 0x0087c0c0 rage::crmtNodeBlend::vf0
/// Deleting destructor for a blend node.
///
/// Stamps the blend vtable (file VA 0x00FE8544, relocated), runs the pair
/// teardown (intercepted callee 1, thiscall/0) over this, stamps the node
/// vtable (file VA 0x00FE8504, relocated), runs the pair teardown again,
/// runs intercepted callee 2 (thiscall/0), and when bit 0 of `flag` is set
/// frees this through the TLS heap (slot 0, allocator at `[slot]+8`, slot
/// 0x0c of its table, this as the only stack argument). Returns this.
///
/// Original: thiscall/1, three direct plus one indirect call, no floats.
export!(thiscall, rw_0087c0c0(this: u32, flag: u32) -> u32 {
    /// Blend-node vtable (file VA; relocated at load).
    const BLEND_VTABLE: u32 = 0x00FE8544;
    /// Plain-node vtable (file VA; relocated at load).
    const NODE_VTABLE: u32 = 0x00FE8504;
    /// Fabricated TLS slot holding the heap anchor.
    const TLS_SLOT: usize = 0;
    /// Free slot in the heap object's table.
    const FREE_SLOT: u32 = 0x0C;
    unsafe {
        (this as *mut u32).write_unaligned(relocated(BLEND_VTABLE));
        callee_thiscall!(1, u32, this);
        (this as *mut u32).write_unaligned(relocated(NODE_VTABLE));
        callee_thiscall!(1, u32, this);
        callee_thiscall!(2, u32, this);
        if flag & 1 != 0 {
            let tls = tls_slot(TLS_SLOT);
            let alloc = ((tls + 8) as *const u32).read_unaligned();
            let vt = (alloc as *const u32).read_unaligned();
            let tgt = ((vt + FREE_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            f(alloc, this);
        }
        this
    }
});
