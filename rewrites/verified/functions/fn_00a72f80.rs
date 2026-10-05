// original: 0x00a72f80 CTaskComplexPlayerOnFoot::vf5 (symbols)

/// Reconsider the on-foot task: poll the interested party, else reset the
/// task's persistent state.
///
/// `thiscall(this, arg0, arg1, arg2)` returning a byte in AL. With a live
/// arg2 it chains that object's and the owner's (`[arg0+0x224]`) virtual
/// slots (`+4/+4`, `+0x20` four times, `+0xC`, `+4`) against the constants
/// `0x42`/`9`/`0x1AF`, storing the final pick at `[this+0x74]` and bumping
/// `[arg2+4]` before returning 0; a set bit in `[arg0+0x270]` with the
/// first answer `0x42` also returns 0 early. Any mismatch, a null arg2, a
/// busy `[this+0x74]` or `arg1==2` takes the long path instead: re-ask the
/// scheduler object at `[this+8]` through virtual slot `+0x14` unless
/// flag bit 1 is set (0 back means return 0), optionally refresh arg0
/// through a direct callee, clear bit 3 of a word in the object at
/// `[arg0+0x228]+0x70`, reset four slots of a file-static table pair
/// (dwords to all-ones, bytes to zero) and this task's own state words,
/// release two owned references and the `[this+0x74]` pick through
/// virtual slot 0, and return 1. The in-loop `i >= 4` fail-check is
/// provably dead (the pointer steps `0x10` while below end, so the check
/// sees `0..4` only) and is not replicated.
///
/// Original: 0x00a72f80 (thiscall, three stack words; AL is the channel).
lf_checker_rt::export!(thiscall, rw_00a72f80(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const VTBL_V1: u32 = 0x04;
        const VTBL_W1: u32 = 0x20;
        const VTBL_X1: u32 = 0x0C;
        const VTBL_V2: u32 = 0x04;
        const VTBL_R: u32 = 0x14;
        const TABLE_PTR: u32 = 0x012FA6F8;
        const TABLE_END: u32 = 0x012FA738;
        const TABLE_WORDS: u32 = 0x012FA6BC;
        const TABLE_BYTES: u32 = 0x012FA6CC;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let vt = rd32(obj);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(slot)) as usize);
                f(obj)
            }
        }

        if a2 != 0 {
            if vcall0(a2, VTBL_V1) == 0x42 && rd32(a0.wrapping_add(0x270)) & 0x20000 != 0 {
                return 0;
            }
            if rd32(this.wrapping_add(0x74)) == 0
                && a1 != 2
                && vcall0(a2, VTBL_V1) == 9
            {
                let w = vcall0(rd32(a0.wrapping_add(0x224)), VTBL_W1);
                if rd32(w.wrapping_add(8)) != 0 {
                    let w = vcall0(rd32(a0.wrapping_add(0x224)), VTBL_W1);
                    if vcall0(rd32(w.wrapping_add(8)), VTBL_X1) == 0x1AF {
                        let w = vcall0(rd32(a0.wrapping_add(0x224)), VTBL_W1);
                        if rd32(w.wrapping_add(0x0C)) == 0 {
                            let w = vcall0(rd32(a0.wrapping_add(0x224)), VTBL_W1);
                            let pick = vcall0(rd32(w.wrapping_add(8)), VTBL_V2);
                            (this.wrapping_add(0x74) as *mut u32).write_unaligned(pick);
                            let c = rd32(a2.wrapping_add(4)).wrapping_add(1);
                            (a2.wrapping_add(4) as *mut u32).write_unaligned(c);
                            return 0;
                        }
                    }
                }
            }
        }
        // Long path: re-ask the scheduler unless flag bit 1 is set.
        let ebp = rd32(this.wrapping_add(8));
        if (ebp.wrapping_add(0x0C) as *const u8).read() & 1 == 0 {
            let vt = rd32(ebp);
            let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VTBL_R)) as usize);
            if f(ebp, a0, a1, a2) as u8 == 0 {
                return 0;
            }
            let fl = rd32(ebp.wrapping_add(0x0C)) | 2;
            (ebp.wrapping_add(0x0C) as *mut u32).write_unaligned(fl);
        }
        if rd32(a0.wrapping_add(0x398)) != 0 {
            lf_checker_rt::callee_thiscall!(7, u32, a0);
        }
        let q = rd32(a0.wrapping_add(0x228));
        let base = if q != 0 { q.wrapping_add(0x70) } else { 0 };
        let w = rd32(base.wrapping_add(0x3D0)) & 0xFFFF_FFF7;
        (base.wrapping_add(0x3D0) as *mut u32).write_unaligned(w);
        // File-static table reset. The original's in-loop `i >= 4`
        // fail-check is provably dead (see doc comment) and is omitted.
        let mut p = lf_checker_rt::relocated(TABLE_PTR);
        let end = lf_checker_rt::relocated(TABLE_END);
        let mut i = 0u32;
        while (p as i32) < (end as i32) {
            lf_checker_rt::callee_thiscall!(8, u32, p);
            lf_checker_rt::global::<u32>(TABLE_WORDS.wrapping_add(i.wrapping_mul(4)))
                .write(0xFFFF_FFFF);
            lf_checker_rt::global::<u8>(TABLE_BYTES.wrapping_add(i)).write(0);
            p = p.wrapping_add(0x10);
            i += 1;
        }
        lf_checker_rt::callee_thiscall!(8, u32, this.wrapping_add(0x18));
        (this.wrapping_add(0x28) as *mut u32).write_unaligned(0xFFFF_FFFF);
        (this.wrapping_add(0x2C) as *mut u8).write(0);
        (this.wrapping_add(0x38) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x34) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x30) as *mut u32).write_unaligned(0);
        let c48 = rd32(this.wrapping_add(0x48));
        (this.wrapping_add(0x40) as *mut u8).write(0);
        (this.wrapping_add(0x44) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0x4C) as *mut u32).write_unaligned(0);
        if c48 != 0 {
            lf_checker_rt::callee_stdcall!(9, u32, this.wrapping_add(0x48));
            (this.wrapping_add(0x48) as *mut u32).write_unaligned(0);
        }
        let c74 = rd32(this.wrapping_add(0x74));
        if c74 != 0 {
            let vt = rd32(c74);
            let f: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(vt) as usize);
            f(c74, 1);
            (this.wrapping_add(0x74) as *mut u32).write_unaligned(0);
        }
        if rd32(this.wrapping_add(0x80)) != 0 {
            lf_checker_rt::callee_stdcall!(9, u32, this.wrapping_add(0x80));
        }
        (this.wrapping_add(0x80) as *mut u32).write_unaligned(0);
        (this.wrapping_add(0xA0) as *mut u32).write_unaligned(0x4120_0000);
        1
    }
});
