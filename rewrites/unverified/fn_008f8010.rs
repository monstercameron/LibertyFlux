// original: 0x008f8010 input_list_clear (proposed)

/// Release every element, free both arrays, and reset the list fields.
///
/// `obj` is the input device object. Each of the UNSIGNED 16-bit count at
/// `[obj+4]` entries of the array at `[obj+0]` that is non-null has its
/// virtual slot at vtable `+0` called (thiscall, ECX = element, one stack
/// word 1); null entries are skipped. Both arrays (`[obj+0]` and `[obj+8]`)
/// are then passed to the free callee (cdecl, one word each) and the fields
/// reset: `[0]`, `[4]` (dword), `[8]`, `[0xC]` to 0, `[0x24]` and `[0x458]`
/// to -1, `[0x45C]` to 0x1F4, and the words and dwords at `0x450`, `0x453`,
/// `0x460`, `0x464`, `0x468`, `0x46C` to 0. EAX is 0 at return.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f8010(obj: u32) -> u32 {
    unsafe {
        const C_VIRT: u32 = 1;
        const C_FREE: u32 = 2;
        const ARRAY_OFF: u32 = 0x0;
        const COUNT_OFF: u32 = 0x4;
        const AUX_OFF: u32 = 0x8;
        const LIST_END: u32 = 0xc;
        const VT_SLOT: u32 = 0x0;
        const fn w32(base: u32, off: u32, v: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(v) }
        }
        let mut i = 0u32;
        loop {
            let n = ((obj + COUNT_OFF) as *const u16).read_unaligned() as u32;
            if i >= n {
                break;
            }
            let array = ((obj + ARRAY_OFF) as *const u32).read_unaligned();
            let elem = ((array + i * 4) as *const u32).read_unaligned();
            if elem != 0 {
                let vt = (elem as *const u32).read_unaligned();
                let target = ((vt + VT_SLOT) as *const u32).read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let _ = f(elem, 1);
            }
            i += 1;
        }
        let a0: u32 = ((obj + ARRAY_OFF) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(C_FREE, u32, a0);
        w32(obj, ARRAY_OFF, 0);
        w32(obj, COUNT_OFF, 0);
        let a1: u32 = ((obj + AUX_OFF) as *const u32).read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(C_FREE, u32, a1);
        w32(obj, AUX_OFF, 0);
        w32(obj, LIST_END, 0);
        ((obj + 0x450) as *mut u16).write_unaligned(0);
        w32(obj, 0x458, 0xffff_ffff);
        w32(obj, 0x45c, 0x1f4);
        w32(obj, 0x464, 0);
        w32(obj, 0x460, 0);
        w32(obj, 0x46c, 0);
        w32(obj, 0x468, 0);
        ((obj + 0x453) as *mut u16).write_unaligned(0);
        w32(obj, 0x24, 0xffff_ffff);
        0
    }
});
