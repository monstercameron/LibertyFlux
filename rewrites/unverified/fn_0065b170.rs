// original: 0x0065B170 tls_free_entry_array (proposed)

/// Free each live entry pointer, then the array itself, via the TLS free.
///
/// Walks `count` entries of 0x18 bytes at `base` (`count` compared as
/// signed: zero and negative skip the loop). An entry whose word at `+0x14`
/// equals its own address `+0x10` (self-linked) or whose pointer at `+0` is
/// null is skipped; every other pointer is freed through the thread-local
/// allocator's free slot (`+0x0c`, thiscall: holder object, pointer). After
/// the loop a non-null `base` is freed the same way. The return value is the
/// last free call's answer, or the entry leftover when no call fires, so no
/// return channel is compared (stdcall, two arguments).
lf_checker_rt::export!(stdcall, rw_0065b170(base: u32, count: u32) -> u32 {
    unsafe {
        const ENTRY: u32 = 0x18;
        let holder = lf_checker_rt::tls_slot(0);
        let frobj = ((holder + 8) as *const u32).read_unaligned();
        let fvt = (frobj as *const u32).read_unaligned();
        let ftgt = ((fvt + 0x0c) as *const u32).read_unaligned();
        let free: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(ftgt as usize);
        let n = count as i32;
        if n > 0 {
            let mut i = 0i32;
            while i < n {
                let e = base.wrapping_add((i as u32).wrapping_mul(ENTRY));
                let link = e.wrapping_add(0x10);
                if ((link + 4) as *const u32).read_unaligned() != link {
                    let p = (e as *const u32).read_unaligned();
                    if p != 0 {
                        free(frobj, p);
                    }
                }
                i += 1;
            }
        }
        if base != 0 {
            free(frobj, base);
        }
        0
    }
});
