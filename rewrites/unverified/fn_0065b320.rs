// original: 0x0065B320 tls_alloc_entry_array (proposed)

/// Allocate an array of `n` 0x18-byte entries through the TLS allocator.
///
/// Calls the allocator (TLS slot 0, slot `+8`, thiscall: object, `3*n*8`,
/// `0x10`, 0). When `n` is positive (compared as signed: zero and negative
/// skip the loop) each entry is linked: the word at entry `+0x10` gets the
/// block address and the word at `+0x14` gets the entry's own address; a
/// null block skips the stores. The multiply and shift wrap mod 2^32.
/// Returns the block (stdcall, one argument).
lf_checker_rt::export!(stdcall, rw_0065b320(n: u32) -> u32 {
    unsafe {
        const ENTRY: u32 = 0x18;
        const HEAD_OFF: u32 = 0x10;
        let holder = lf_checker_rt::tls_slot(0);
        let obj = ((holder + 8) as *const u32).read_unaligned();
        let vtable = (obj as *const u32).read_unaligned();
        let tgt = ((vtable + 8) as *const u32).read_unaligned();
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let size = n.wrapping_mul(3).wrapping_shl(3);
        let mem = alloc(obj, size, 0x10, 0);
        let count = n as i32;
        if count > 0 && mem != 0 {
            let mut i = 0i32;
            while i < count {
                let node = mem.wrapping_add(HEAD_OFF).wrapping_add((i as u32).wrapping_mul(ENTRY));
                (node as *mut u32).write_unaligned(mem);
                ((node + 4) as *mut u32).write_unaligned(node);
                i += 1;
            }
        }
        mem
    }
});
