// original: 0x00ca52e0 CEventHandler::vf64
/// Dual-path float event slot: bit 1 of the byte at event+0x38 selects one
/// of two sibling converters; both take the same seven-word vector block
/// (three event-relative pointers, three words, one float) whose first word
/// is replaced by a freshly evaluated scalar. Stores the result at this+0xC.
lf_rs75_rt::export!(thiscall, rw_00ca52e0(this: u32, ev: u32, _b: u32, _c: u32) -> u32 {
    unsafe {
        let second = (*((ev + 0x38) as *const u8) & 2) == 0;
        let mgr = *lf_rs75_rt::global::<u32>(0x0167E2A0);
        let h: u32 = lf_rs75_rt::callee_thiscall!(1, u32, mgr);
        if h == 0 {
            *((this + 0xC) as *mut u32) = 0;
            return 0;
        }
        let p10 = ev.wrapping_add(0x10);
        let p20 = ev.wrapping_add(0x20);
        let p50 = ev.wrapping_add(0x50);
        let f30 = *((ev + 0x30) as *const u32);
        let w34 = *((ev + 0x34) as *const u32);
        let w3c = *((ev + 0x3C) as *const u32);
        let w40 = *((ev + 0x40) as *const u32);
        let scalar: f32 = lf_rs75_rt::callee_cdecl!(2, f32, w34, p10, f30, p20, w3c, w40, p50);
        let ans: u32 = if second {
            lf_rs75_rt::callee_thiscall!(4, u32, h, scalar.to_bits(), p10, f30, p20, w3c, w40, p50)
        } else {
            lf_rs75_rt::callee_thiscall!(3, u32, h, scalar.to_bits(), p10, f30, p20, w3c, w40, p50)
        };
        *((this + 0xC) as *mut u32) = ans;
        ans
    }
});
