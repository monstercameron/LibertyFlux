// original: 0x0092CFE0 T_CB_Generic_3Args<void(*)(int, const rage::Matrix44&, float), int, rage::Matrix44, float>::vf1 (symbols)

/// Virtual slot 1 of the 3-argument generic callback: invoke the stored function.
///
/// `this` holds a function pointer at `+0x08`, an int at `+0x0c`, a 4x4
/// matrix at `+0x10` and a float at `+0x50`. Calls the stored function
/// (callee 1, register-indirect) with `(int, &matrix, float)` and returns
/// its result. The matrix is passed by address (`this + 0x10`).
///
/// Original: 0x0092CFE0 (thiscall, no stack arguments). One indirect call.
lf_checker_rt::export!(thiscall, rw_0092CFE0(this: u32) -> u32 {
    unsafe {
        const FUNC_OFF: u32 = 0x08;
        const ARG0_OFF: u32 = 0x0c;
        const MATRIX_OFF: u32 = 0x10;
        const FLOAT_OFF: u32 = 0x50;
        let f = (this.wrapping_add(FLOAT_OFF) as *const u32).read_unaligned();
        let m = this.wrapping_add(MATRIX_OFF);
        let a = (this.wrapping_add(ARG0_OFF) as *const u32).read_unaligned();
        let fp = (this.wrapping_add(FUNC_OFF) as *const u32).read_unaligned();
        let func: extern "cdecl" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(fp as usize);
        func(a, m, f)
    }
});
