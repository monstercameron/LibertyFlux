// original: 0x0061DE30 net_install_global_ctx

/// Install the global network context record.
///
/// Stamps the context globals (flag bit, state words, three relocated
/// table pointers), fetches the allocator block, links it into the
/// context (`[block]` points back at the context), bumps the block's
/// two counters (and a third when the state word exceeds the block's
/// limit, which cannot happen with the just-stored state), swaps the
/// context's table pointer, and returns the context address.
/// Original: 0x0061DE30 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_0061DE30() -> u32 {
    unsafe {
        const CTX: u32 = 0x019F0740;
        const ALLOC_CALLEE: u32 = 1;
        let gb = lf_checker_rt::relocated(CTX);
        let flag = ((gb + 0x1C) as *mut u8).read();
        ((gb + 0x1C) as *mut u8).write(flag | 1);
        ((gb + 4) as *mut u32).write_unaligned(1);
        ((gb + 8) as *mut u32).write_unaligned(0);
        ((gb + 0x10) as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00FE1F84));
        ((gb + 0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(0x019F0800));
        ((gb + 0x18) as *mut u32).write_unaligned(0);
        ((gb) as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00FE2264));
        let block = lf_checker_rt::callee_cdecl!(ALLOC_CALLEE, u32,);
        ((gb + 0x18) as *mut u32).write_unaligned((block as *const u32).read_unaligned());
        (block as *mut u32).write_unaligned(gb);
        if (((gb + 0x1C) as *const u8).read() & 1) != 0 {
            let state = ((gb + 8) as *const u32).read_unaligned();
            let limit = ((block + 0x10) as *const u32).read_unaligned();
            if state > limit {
                let v = ((block + 0x10) as *const u32).read_unaligned();
                ((block + 0x10) as *mut u32).write_unaligned(v.wrapping_add(1));
            }
            let v = ((block + 0xC) as *const u32).read_unaligned();
            ((block + 0xC) as *mut u32).write_unaligned(v.wrapping_add(1));
        }
        let v = ((block + 8) as *const u32).read_unaligned();
        ((block + 8) as *mut u32).write_unaligned(v.wrapping_add(1));
        (gb as *mut u32).write_unaligned(lf_checker_rt::relocated(0x00FE2228));
        gb
    }
});
