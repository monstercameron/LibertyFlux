// original: 0x00887B00 stream_object_init (proposed)

/// Initialise a stream object with its name and links.
///
/// Stores `a1` at `+0xc`, `a2` at `+0x10`, `a0` at `+0x4` and fixed
/// sentinels through the header; copies the name `a3` (when non-null) into
/// a fresh block (callee 1) with the copy entry (callee 2) and stores the
/// block (or null) at `+0x0`; builds two lock words (callee 3, twice) and
/// links the object (callee 4). The answer is `this`.
///
/// Original: 0x00887B00 (thiscall, four stack words).
lf_checker_rt::export!(thiscall, rw_00887B00(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const NAME: u32 = 0x00;
        const LINK0: u32 = 0x04;
        const LINK1: u32 = 0x08;
        const P0: u32 = 0x0c;
        const P1: u32 = 0x10;
        const LOCK_A: u32 = 0x34;
        const LOCK_B: u32 = 0x38;
        const ALLOC: u32 = 1;
        const COPY: u32 = 2;
        const MAKE_LOCK: u32 = 3;
        const LINK: u32 = 4;
        // NOTE: the export macro passes the first app argument in ecx, so
        // the parameter order here is (this, a0..a3) while the original
        // reads [esp+4]=a0, [esp+8]=a1, [esp+0xc]=a2, [esp+0x10]=a3.
        ((this + P0) as *mut u32).write_unaligned(a1);
        ((this + P1) as *mut u32).write_unaligned(a2);
        ((this + 0x44) as *mut u32).write_unaligned(0xffff);
        ((this + 0x2c) as *mut u32).write_unaligned(0xffff_ffff);
        ((this + 0x14) as *mut u32).write_unaligned(0);
        ((this + 0x42) as *mut u16).write_unaligned(0xffff);
        ((this + 0x18) as *mut u32).write_unaligned(0);
        ((this + 0x1c) as *mut u32).write_unaligned(0);
        ((this + 0x49) as *mut u16).write_unaligned(0);
        let block = if a3 == 0 {
            0
        } else {
            let mut n = 0u32;
            while ((a3.wrapping_add(n)) as *const u8).read() != 0 {
                n = n.wrapping_add(1);
            }
            n = n.wrapping_add(1);
            let b = lf_checker_rt::callee_cdecl!(ALLOC, u32, n, 1);
            // The copy entry returns its destination, so its answer is the block.
            lf_checker_rt::callee_cdecl!(COPY, u32, b, a3, n)
        };
        ((this + NAME) as *mut u32).write_unaligned(block);
        ((this + LINK0) as *mut u32).write_unaligned(a0);
        ((this + LINK1) as *mut u32).write_unaligned(0);
        let la = lf_checker_rt::callee_cdecl!(MAKE_LOCK, u32, 0);
        ((this + LOCK_A) as *mut u32).write_unaligned(la);
        let lb = lf_checker_rt::callee_cdecl!(MAKE_LOCK, u32, 1);
        ((this + LOCK_B) as *mut u32).write_unaligned(lb);
        let l0 = ((this + P0) as *const u32).read_unaligned();
        let l1 = ((this + LINK0) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(LINK, u32, l1, 0, l0);
        ((this + 0x3c) as *mut u32).write_unaligned(0);
        ((this + 0x48) as *mut u8).write(0);
        ((this + 0x28) as *mut u32).write_unaligned(1);
        ((this + 0x24) as *mut u32).write_unaligned(0);
        this
    }
});
