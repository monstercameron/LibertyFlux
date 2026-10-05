// original: 0x0093f470 stream_slots_apply (proposed)

/// Call `func` for each live streaming slot, top-down, until it declines.
///
/// `this` points at the slot set: element array at +0x00, flag bytes at
/// +0x04, count at +0x08, element stride at +0x0C. Slots are visited from
/// the top down; a slot whose flag byte has bit 0x80 set is skipped, as is
/// one whose element address computes to null. `func(addr, arg)` runs for
/// the rest and a zero answer stops the walk. The return value is the last
/// callback answer, or entry garbage when no slot ran, so it is not
/// compared.
///
/// Original: 0x0093f470 (thiscall, two stack words; one register callee).
lf_checker_rt::export!(thiscall, rw_0093f470(this: u32, func: u32, arg: u32) -> u32 {
    const SET_ITEMS: u32 = 0x00;
    const SET_FLAGS: u32 = 0x04;
    const SET_COUNT: u32 = 0x08;
    const SET_STRIDE: u32 = 0x0C;
    const SKIP_FLAG: u8 = 0x80;
    unsafe {
        let count = ((this + SET_COUNT) as *const u32).read_unaligned();
        if count == 0 {
            return 0;
        }
        let mut i = count;
        loop {
            i -= 1;
            let flags = ((this + SET_FLAGS) as *const u32).read_unaligned();
            if ((flags + i) as *const u8).read() & SKIP_FLAG == 0 {
                let stride = ((this + SET_STRIDE) as *const u32).read_unaligned();
                let base = ((this + SET_ITEMS) as *const u32).read_unaligned();
                let addr = base.wrapping_add(i.wrapping_mul(stride));
                if addr != 0 {
                    let f: extern "cdecl" fn(u32, u32) -> u32 =
                        core::mem::transmute(func as usize);
                    if f(addr, arg) == 0 {
                        return 0;
                    }
                }
            }
            if i == 0 {
                break;
            }
        }
        0
    }
});
