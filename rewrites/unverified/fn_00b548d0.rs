// original: 0x00B548D0 crmt_entry_create (proposed)

/// Create a list entry from a config word and append it to the list.
///
/// Allocates the entry through the allocator whose pointer the global at
/// 0x1669D64 holds (callee 1); a null answer falls through and faults on
/// the first initialising store, like the original. Otherwise the entry
/// is zeroed, stamped with its table address, given tag 1 and the config
/// word from `cfg` + 0x14 (sign-extended by construction, passed to the
/// slot counter at callee 2), linked to `cfg`, and appended to the list
/// at `this` + 0x1A34 by callee 3, whose answer is returned.
///
/// Original: 0x00B548D0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b548d0(this: u32, cfg: u32) -> u32 {
    unsafe {
        const ALLOC_G: u32 = 0x1669d64;
        const VTABLE: u32 = 0xeaf3b8;
        const LIST_OFF: u32 = 0x1a34;
        const ALLOC: u32 = 1;
        const BUMP: u32 = 2;
        const PUSH: u32 = 3;
        let ator = lf_checker_rt::global::<u32>(ALLOC_G).read_unaligned();
        let obj: u32 = lf_checker_rt::callee_thiscall!(ALLOC, u32, ator);
        if obj != 0 {
            let mut off = 4u32;
            while off <= 0x14 {
                ((obj + off) as *mut u32).write_unaligned(0);
                off += 4;
            }
            (obj as *mut u32).write_unaligned(VTABLE);
        }
        ((obj + 4) as *mut u16).write_unaligned(1);
        let w = ((cfg + 0x14) as *const u16).read_unaligned();
        ((obj + 6) as *mut u16).write_unaligned(w);
        let arg = (w as i16 as i32) as u32;
        ((obj + 8) as *mut u32).write_unaligned(cfg);
        ((obj + 0xc) as *mut u32).write_unaligned(0);
        let _: u32 = lf_checker_rt::callee_cdecl!(BUMP, u32, arg);
        lf_checker_rt::callee_thiscall!(PUSH, u32, this + LIST_OFF, obj)
    }
});
