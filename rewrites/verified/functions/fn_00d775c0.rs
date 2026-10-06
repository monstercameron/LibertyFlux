// original: 0x00D775C0 T_CB_Generic_4Args<void(*)(float, int, int, int), float, int, int, int>::vf1

/// Invoke the stored `void(float, int, int, int)` callback.
///
/// `this + 8` holds the function pointer; the arguments come from
/// `+0xc` (float bits, passed through `xmm0` to the stack top), `+0x10`,
/// `+0x14` and `+0x18`. Cdecl callback, caller cleans four words.
/// Returns the callback's answer. Thiscall: object in `ecx`.
use lf_checker_rt::export;

export!(thiscall, rw_00d775c0(this: u32) -> u32 {
    unsafe {
        const CB_OFF: u32 = 8;
        const A0_OFF: u32 = 0x0c;
        const A1_OFF: u32 = 0x10;
        const A2_OFF: u32 = 0x14;
        const A3_OFF: u32 = 0x18;
        let func = ((this + CB_OFF) as *const u32).read_unaligned();
        let f = ((this + A0_OFF) as *const u32).read_unaligned();
        let a = ((this + A1_OFF) as *const u32).read_unaligned();
        let bb = ((this + A2_OFF) as *const u32).read_unaligned();
        let cc = ((this + A3_OFF) as *const u32).read_unaligned();
        let cb: extern "cdecl" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(func as usize);
        cb(f, a, bb, cc)
    }
});
