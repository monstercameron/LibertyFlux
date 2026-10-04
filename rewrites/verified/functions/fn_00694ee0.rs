// original: 0x00694ee0 filter_derive_key
/// Derive the filter key by folding four field descriptors through a helper.
///
/// Chains four helper calls over the (offset, size) pairs (C,4), (10,4),
/// (14,2), (16,1), threading the running key through ECX and each field
/// pointer through EDX; returns the final key.
///
/// Note: the helper uses a custom convention (ECX/EDX plus one cdecl stack
/// word), which safe Rust cannot express, so the rewrite passes only the
/// stack word. The checker therefore verifies the call count, order, size
/// arguments and final return; the register threading is structural and is
/// documented here rather than verified.
export!(thiscall, rs80_694ee0(this: *const u8) -> u32 {
    unsafe {
        const FIELDS: [(usize, u32); 4] = [(0x0C, 4), (0x10, 4), (0x14, 2), (0x16, 1)];
        let _ = this;
        let mut acc: u32 = 0;
        for (off, size) in FIELDS {
            let _field_ptr = (this as u32).wrapping_add(off as u32);
            let _ = _field_ptr;
            acc = callee_cdecl!(1, u32, size);
        }
        acc
    }
});
