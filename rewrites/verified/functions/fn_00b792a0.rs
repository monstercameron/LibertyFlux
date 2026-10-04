// original: 0x00b792a0 task_chain_float_set
/// Walk a task chain, pushing a float into each node and clearing a flag.
///
/// For each node until the null link at offset `0x8`: probes it through
/// vtable slot `0x8`, forwards the float argument plus zero to the embedded
/// subobject at `0x14` through its vtable slot `0x4`, fetches a state block
/// through vtable slot `0x30` and clears its bit 0 at offset `0x8`.
/// Returns the last fetched state block.
export!(thiscall, rw_00b792a0(this: u32, arg: f32) -> u32 {
    let mut node = this;
    loop {
        let vt = unsafe { *(node as *const u32) };
        let probe: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(*((vt + 8) as *const u32) as usize) };
        let _ = probe(node);
        let inner = node.wrapping_add(0x14);
        let ivt = unsafe { *(inner as *const u32) };
        let set: extern "thiscall" fn(u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(*((ivt + 4) as *const u32) as usize) };
        let _ = set(inner, arg.to_bits(), 0);
        let vt2 = unsafe { *(node as *const u32) };
        let fetch: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(*((vt2 + 0x30) as *const u32) as usize) };
        let p = fetch(node);
        unsafe { *((p + 8) as *mut u32) &= !1; }
        let next = unsafe { *((node + 8) as *const u32) };
        if next == 0 {
            return p;
        }
        node = next;
    }
});
