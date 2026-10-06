// original: 0x008f7d00 input_list_update_flags (proposed)

/// Update every element's state flags, notifying through callees.
///
/// `obj` is the input device object. For each of the UNSIGNED 16-bit count
/// at `[obj+4]` elements of the array at `[obj+0]`, selected by the flag
/// byte at element `+0x558` and the word at `+0x404`: when bit 0 is set and
/// (bit 1 clear with a zero word) the element's virtual slot at vtable
/// `+0x30` runs (thiscall, ECX = element, one stack word 0); when bit 0 is
/// set the touch callee runs; otherwise, when bit 0 is clear but bit 1 is
/// set, the release callee runs. Every visited element then has bit 1 of
/// its flag byte replaced by its own bit 0 (other bits unchanged). The
/// contract pins flag bytes across all four bit combinations. EAX holds the
/// count at return.
///
/// Thiscall: object in ECX, no stack words.
lf_checker_rt::export!(thiscall, rw_008f7d00(obj: u32) -> u32 {
    unsafe {
        const C_VIRT: u32 = 1;
        const C_TOUCH: u32 = 2;
        const C_RELEASE: u32 = 3;
        const ARRAY_OFF: u32 = 0x0;
        const COUNT_OFF: u32 = 0x4;
        const FLAG_OFF: u32 = 0x558;
        const WORD_OFF: u32 = 0x404;
        const VT_SLOT: u32 = 0x30;
        const ACTIVE_BIT: u8 = 1;
        const ARMED_BIT: u8 = 2;
        let mut i = 0u32;
        loop {
            let n = ((obj + COUNT_OFF) as *const u16).read_unaligned() as u32;
            if i >= n {
                return n;
            }
            let array = ((obj + ARRAY_OFF) as *const u32).read_unaligned();
            let elem = ((array + i * 4) as *const u32).read_unaligned();
            let f = ((elem + FLAG_OFF) as *const u8).read();
            if f & ACTIVE_BIT != 0 {
                if f & ARMED_BIT == 0
                    && ((elem + WORD_OFF) as *const u16).read_unaligned() == 0
                {
                    let vt = (elem as *const u32).read_unaligned();
                    let target = ((vt + VT_SLOT) as *const u32).read_unaligned();
                    let vf: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(target as usize);
                    let _ = vf(elem, 0);
                }
                let _: u32 = lf_checker_rt::callee_thiscall!(C_TOUCH, u32, elem);
            } else if f & ARMED_BIT != 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(C_RELEASE, u32, elem);
            }
            let g = ((elem + FLAG_OFF) as *const u8).read();
            ((elem + FLAG_OFF) as *mut u8).write((g & !ARMED_BIT) | ((g & ACTIVE_BIT) << 1));
            i += 1;
        }
    }
});
