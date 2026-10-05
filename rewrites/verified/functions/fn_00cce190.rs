// original: 0x00cce190 CTaskComplexOnFire::vf5
/// Run the on-fire task's shutdown, returning 1 on success, else 0.
///
/// The third stack argument selects an optional pre-check: when non-null
/// its virtual slot at `+4` is asked twice and its slot at `+0x18` once;
/// only a non-zero final answer lets a double-0x3a pass, any other shape
/// falls through to the shared shutdown below (a first answer other than
/// 0x3a or a second equal to 0x78 skips the final ask). Then the subtask
/// at `this+8`, unless its flag byte at `+0xc` is already set, is shut
/// down through its slot at `+0x14` (thiscall, all three stack arguments);
/// a zero answer fails with 0, otherwise bit 1 of the flag is set and the
/// call returns 1 in AL. Thiscall with three stack arguments.
export!(thiscall, rw_00cce190(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x08;
        const FLAG_OFF: u32 = 0x0c;
        const VT_SLOT: u32 = 0x14;
        const DONE_BIT: u32 = 2;
        if a2 != 0 {
            let vt = (a2 as *const u32).read_unaligned();
            let s4 = (vt.wrapping_add(4) as *const u32).read_unaligned();
            let f4: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(s4 as usize);
            if f4(a2) == 0x3a {
                let vt2 = (a2 as *const u32).read_unaligned();
                let s4b = (vt2.wrapping_add(4) as *const u32).read_unaligned();
                let f4b: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(s4b as usize);
                if f4b(a2) != 0x78 {
                    let s18 = (vt2.wrapping_add(0x18) as *const u32).read_unaligned();
                    let f18: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(s18 as usize);
                    if f18(a2) & 0xff == 0 {
                        return 0;
                    }
                }
            }
        }
        let sub = (this.wrapping_add(SUB_OFF) as *const u32).read_unaligned();
        if (sub.wrapping_add(FLAG_OFF) as *const u8).read() & 1 == 0 {
            let vtab = (sub as *const u32).read_unaligned();
            let slot = (vtab.wrapping_add(VT_SLOT) as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(slot as usize);
            if f(sub, a0, a1, a2) & 0xff == 0 {
                return 0;
            }
            let fl = (sub.wrapping_add(FLAG_OFF) as *const u32).read_unaligned();
            (sub.wrapping_add(FLAG_OFF) as *mut u32).write_unaligned(fl | DONE_BIT);
        }
        1
    }
});
