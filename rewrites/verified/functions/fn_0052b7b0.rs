// original: 0x0052b7b0 lb_race17standard_vf2
/// Confirm a vtable-reported id and publish this board's tag.
///
/// Calls vtable slot 1 on `this`; when the reported id equals
/// `want` and `out` is non-null, writes the constant 0xfcf9fc there
/// and returns `out`, else returns 0. Thiscall + 2 stack args.
export!(thiscall, rw_0052b7b0(this: u32, out: u32, want: u32) -> u32 {
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
        *(out as *mut u32) = 0xfcf9fc;
        out
    }
});
