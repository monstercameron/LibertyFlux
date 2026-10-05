// original: 0x00beb8f0 CPedInterp::vf0
/// Resolve an interpolation target through two lookups and a solver call.
///
/// Stores `arg` at `[this+8]`, then dispatches on its tag byte at +0x17:
/// 'P' adds 0x370 to `[this+0x10]`, 'Z' adds 0x398, anything else adds
/// nothing. Calls the lookup (thiscall on `arg`, no stack args, intercepted)
/// twice: a first answer of 0xffffff zeroes `[this+0x14]` and `[this+0xc]`
/// and returns 0xffffff; a second answer of 0 zeroes and returns 0.
/// Otherwise builds a 12-byte frame from the shared qword at file VA
/// 0x0120CA30 and dword at 0x0120CA38, passes its address plus `arg` to the
/// solver (cdecl, intercepted; it fills two words), stores their sum at
/// `[this+0xc]`, and dispatches on the sum's tag byte at +0x17 the same way
/// ('P' +0x370, 'Z' +0x398) into `[this+0x14]`. Returns the sum with its low
/// byte replaced by that tag byte (the original reloads AL from it).
/// Thiscall, one stack argument.
export!(thiscall, rw_00beb8f0(this: u32, arg: u32) -> u32 {
    unsafe {
        const G_QWORD: u32 = 0x0120CA30;
        const G_DWORD: u32 = 0x0120CA38;
        const LOOKUP_FIRST: u32 = 1;
        const LOOKUP_SECOND: u32 = 2;
        const SOLVER: u32 = 3;
        const MISSING: u32 = 0xFFFFFF;
        let g2lo = (relocated(G_QWORD) as *const u32).read_unaligned();
        let g2hi = (relocated(G_QWORD + 4) as *const u32).read_unaligned();
        let g1 = (relocated(G_DWORD) as *const u32).read_unaligned();
        ((this + 8) as *mut u32).write_unaligned(arg);
        let tag = ((arg + 0x17) as *const u8).read();
        if tag == 0x50 {
            let v = ((this + 0x10) as *const u32).read_unaligned();
            ((this + 0x10) as *mut u32).write_unaligned(v.wrapping_add(0x370));
        } else if tag == 0x5A {
            let v = ((this + 0x10) as *const u32).read_unaligned();
            ((this + 0x10) as *mut u32).write_unaligned(v.wrapping_add(0x398));
        }
        let r1: u32 = callee_thiscall!(LOOKUP_FIRST, u32, arg);
        if r1 == MISSING {
            ((this + 0x14) as *mut u32).write_unaligned(0);
            ((this + 0x0c) as *mut u32).write_unaligned(0);
            return r1;
        }
        let r2: u32 = callee_thiscall!(LOOKUP_SECOND, u32, arg);
        if r2 == 0 {
            return 0;
        }
        let mut frame = [g2lo, g2hi, g1];
        let _: u32 = callee_cdecl!(SOLVER, u32, frame.as_mut_ptr() as u32, arg);
        let sum = frame[0].wrapping_add(frame[1]);
        ((this + 0x0c) as *mut u32).write_unaligned(sum);
        let tag2 = ((sum + 0x17) as *const u8).read();
        if tag2 == 0x50 {
            let v = ((this + 0x14) as *const u32).read_unaligned();
            ((this + 0x14) as *mut u32).write_unaligned(v.wrapping_add(0x370));
        } else if tag2 == 0x5A {
            let v = ((this + 0x14) as *const u32).read_unaligned();
            ((this + 0x14) as *mut u32).write_unaligned(v.wrapping_add(0x398));
        }
        (sum & 0xFFFF_FF00) | tag2 as u32
    }
});
