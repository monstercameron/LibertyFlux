// original: 0x00c24720 cam_source_resolve (proposed)
/// Resolve the two blend-source pointers into `out0`/`out1`. Each side is
/// a lazy lookup: a set flag word selects the inline object field (+0x10),
/// else a cached pointer is reused, else a fresh object is allocated (0x80
/// bytes) and constructed, caching the constructor's answer (a failed
/// allocation caches and yields null). Note the second side tests the
/// FIRST side's cache word (+0x154) but reads its own (+0x158), and that
/// quirk is reproduced. Returns `out1`.
///
/// Original: 0x00c24720 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00c24720(obj: u32, out0: u32, out1: u32) -> u32 {
    unsafe {
        const ALLOC_SIZE: u32 = 0x80;
        let c0: u32;
        if (obj.wrapping_add(0x15c) as *const u32).read_unaligned() != 0 {
            c0 = (obj.wrapping_add(0x14c) as *const u32)
                .read_unaligned()
                .wrapping_add(0x10);
        } else if (obj.wrapping_add(0x154) as *const u32).read_unaligned() != 0 {
            c0 = (obj.wrapping_add(0x154) as *const u32).read_unaligned();
        } else {
            let p: u32 = lf_checker_rt::callee_cdecl!(1, u32, ALLOC_SIZE);
            if p == 0 {
                (obj.wrapping_add(0x154) as *mut u32).write_unaligned(0);
                c0 = 0;
            } else {
                let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, p);
                (obj.wrapping_add(0x154) as *mut u32).write_unaligned(r);
                c0 = r;
            }
        }
        (out0 as *mut u32).write_unaligned(c0);
        let c1: u32;
        if (obj.wrapping_add(0x160) as *const u32).read_unaligned() != 0 {
            c1 = (obj.wrapping_add(0x150) as *const u32)
                .read_unaligned()
                .wrapping_add(0x10);
        } else if (obj.wrapping_add(0x154) as *const u32).read_unaligned() != 0 {
            // Quirk, as in the original: tests +0x154, reads +0x158.
            c1 = (obj.wrapping_add(0x158) as *const u32).read_unaligned();
        } else {
            let p: u32 = lf_checker_rt::callee_cdecl!(1, u32, ALLOC_SIZE);
            if p == 0 {
                (obj.wrapping_add(0x158) as *mut u32).write_unaligned(0);
                c1 = 0;
            } else {
                let r: u32 = lf_checker_rt::callee_thiscall!(2, u32, p);
                (obj.wrapping_add(0x158) as *mut u32).write_unaligned(r);
                c1 = r;
            }
        }
        (out1 as *mut u32).write_unaligned(c1);
        out1
    }
});
