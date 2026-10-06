// original: 0x00962180 slot_release
/// Release a two-word handle slot, clearing the target's state fields first.
///
/// `obj` points to the slot: a pointer at `+0` and a flag byte at `+4`.
/// When the flag is set the slot holds a handle whose referent is reached
/// through one indirection (`[[obj]]`); when the flag is clear the pointer
/// is used directly (`[obj]`). If a non-null target is found its words at
/// `+0xA4`/`+0xA8`/`+0xAC` are reset to (`-1`, `0`, `0`) and it is returned;
/// otherwise null is returned. The slot itself is always cleared.
/// Thiscall: object in ECX, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00962180(obj: u32) -> u32 {
    unsafe {
        const TARGET_A4: u32 = 0xa4;
        const TARGET_A8: u32 = 0xa8;
        const TARGET_AC: u32 = 0xac;
        let flag = (obj.wrapping_add(4) as *const u8).read();
        let mut out = 0u32;
        if flag != 0 {
            let p = (obj as *const u32).read_unaligned();
            if p != 0 {
                let q = (p as *const u32).read_unaligned();
                if q != 0 {
                    (q.wrapping_add(TARGET_AC) as *mut u32).write_unaligned(0);
                    (q.wrapping_add(TARGET_A8) as *mut u32).write_unaligned(0);
                    (q.wrapping_add(TARGET_A4) as *mut u32).write_unaligned(0xffff_ffff);
                    out = q;
                }
            }
        } else {
            let p = (obj as *const u32).read_unaligned();
            if p != 0 {
                (p.wrapping_add(TARGET_AC) as *mut u32).write_unaligned(0);
                (p.wrapping_add(TARGET_A8) as *mut u32).write_unaligned(0);
                (p.wrapping_add(TARGET_A4) as *mut u32).write_unaligned(0xffff_ffff);
                out = p;
            }
        }
        (obj.wrapping_add(4) as *mut u8).write(0);
        (obj as *mut u32).write_unaligned(0);
        out
    }
});
