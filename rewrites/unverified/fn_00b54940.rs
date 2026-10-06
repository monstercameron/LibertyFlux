// original: 0x00B54940 crmt_find_or_create (proposed)

/// Find the live entry for `cfg`, or create and append one.
///
/// Scans the list at `this` + 0x1A34 (next at +0x10) for the entry whose
/// word at +0x8 equals `cfg` and whose word at +0x4 has bit 0 set, both
/// exact tests on unsigned values. On a hit the entry is scored (callee
/// 2 takes the sign-extended word at +0x6), detached (callee 3) and
/// released through its table slot 0 with argument 1 (callee 4), whose
/// answer is returned; the original's null retest after detaching can
/// never fire. On a miss an entry is allocated (callee 1 on the
/// allocator the global at 0x1669D64 holds); a null answer falls through
/// and faults on the first store, like the original. Otherwise the entry
/// is zeroed, stamped, tagged 2 or 6 from bit 17 of `cfg` + 0x4, chained
/// to the dependent at `cfg` + 0x84 (notified through its table slot 1,
/// callee 5), counted (callee 6 takes the sign-extended word at +0x6)
/// and appended (callee 7), whose answer is returned.
///
/// Original: 0x00B54940 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b54940(this: u32, cfg: u32) -> u32 {
    unsafe {
        const LIST_OFF: u32 = 0x1a34;
        const ALLOC_G: u32 = 0x1669d64;
        const VTABLE: u32 = 0xeaf3b8;
        const ALLOC: u32 = 1;
        const SCORE: u32 = 2;
        const DETACH: u32 = 3;
        const BUMP: u32 = 6;
        const PUSH: u32 = 7;
        let list = this + LIST_OFF;
        let mut node = (list as *const u32).read_unaligned();
        let mut hit = 0u32;
        while node != 0 {
            let key = ((node + 8) as *const u32).read_unaligned();
            if key == cfg {
                let flags = ((node + 4) as *const u32).read_unaligned();
                if (flags & 1) != 0 {
                    hit = node;
                    break;
                }
            }
            node = ((node + 0x10) as *const u32).read_unaligned();
        }
        if hit != 0 {
            let w = ((hit + 6) as *const u16).read_unaligned();
            let _: u32 =
                lf_checker_rt::callee_cdecl!(SCORE, u32, (w as i16 as i32) as u32);
            let _: u32 = lf_checker_rt::callee_thiscall!(DETACH, u32, list, hit);
            if hit != 0 {
                let vt = (hit as *const u32).read_unaligned();
                let fptr = (vt as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(fptr as usize);
                return f(hit, 1);
            }
            return 0;
        }
        let ator = lf_checker_rt::global::<u32>(ALLOC_G).read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, ator);
        if obj != 0 {
            let mut off = 4u32;
            while off <= 0x14 {
                ((obj + off) as *mut u32).write_unaligned(0);
                off += 4;
            }
            (obj as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        }
        ((obj + 4) as *mut u16).write_unaligned(2);
        let mode = ((cfg + 4) as *const u32).read_unaligned();
        if ((mode >> 0x11) & 1) != 0 {
            ((obj + 4) as *mut u16).write_unaligned(6);
        }
        let w = ((cfg + 0x14) as *const u16).read_unaligned();
        ((obj + 6) as *mut u16).write_unaligned(w);
        ((obj + 8) as *mut u32).write_unaligned(0);
        let dep = ((cfg + 0x84) as *const u32).read_unaligned();
        ((obj + 0xc) as *mut u32).write_unaligned(dep);
        let vt = (dep as *const u32).read_unaligned();
        let fptr = ((vt + 4) as *const u32).read_unaligned();
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(fptr as usize);
        let _: u32 = f(dep);
        let _: u32 = lf_checker_rt::callee_cdecl!(BUMP, u32, (w as i16 as i32) as u32);
        lf_checker_rt::callee_thiscall!(PUSH, u32, list, obj)
    }
});
