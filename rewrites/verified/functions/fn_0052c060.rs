// original: 0x0052c060 lb_race19standard_vf2
/// Confirm a vtable-reported id and publish this board's tag.
///
/// Calls vtable slot 1 on `this`; when the reported id equals
/// `want` and `out` is non-null, writes the constant 0xfdb6c4 there
/// and returns `out`, else returns 0. Thiscall + 2 stack args.
export!(thiscall, rw_0052c060(this: u32, out: u32, want: u32) -> u32 {
    unsafe {
        let vtable = *(this as *const u32);
        let slot = *((vtable.wrapping_add(4)) as *const u32);
        let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(slot as usize);
        let got = f(this);
        if got != want {
            return 0;
        }
        if out == 0 {
            return 0;
        }
        *(out as *mut u32) = 0xfdb6c4;
        out
    }
});
