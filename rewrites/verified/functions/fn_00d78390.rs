// original: 0x00D78390 T_CB_Generic_4Args<void(*)(int, int, bool, bool), int, int, bool, bool>::vf1

/// Invoke the stored `void(int, int, bool, bool)` callback.
///
/// `this + 8` holds the function pointer; the arguments come from
/// `+0xc` and `+0x10` (words) plus the bytes at `+0x14` and `+0x15`,
/// zero-extended. Cdecl callback, caller cleans four words. Returns the
/// callback's answer. Thiscall: object in `ecx`.
use lf_checker_rt::export;

export!(thiscall, rw_00d78390(this: u32) -> u32 {
    unsafe {
        const CB_OFF: u32 = 8;
        const A0_OFF: u32 = 0x0c;
        const A1_OFF: u32 = 0x10;
        const B0_OFF: u32 = 0x14;
        const B1_OFF: u32 = 0x15;
        let func = ((this + CB_OFF) as *const u32).read_unaligned();
        let a = ((this + A0_OFF) as *const u32).read_unaligned();
        let bb = ((this + A1_OFF) as *const u32).read_unaligned();
        let c = ((this + B0_OFF) as *const u8).read() as u32;
        let d = ((this + B1_OFF) as *const u8).read() as u32;
        let cb: extern "cdecl" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(func as usize);
        cb(a, bb, c, d)
    }
});
