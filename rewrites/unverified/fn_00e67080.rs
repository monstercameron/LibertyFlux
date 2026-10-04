// original: 0x00e67080 sweep_pair_call5_then_const (proposed)
/// Call two thiscall/0 callees 5 times over strided addresses, then a const call.
///
/// For `i` in 0..5 (edi counts down from 4 while non-negative) calls callee 1
/// then callee 2 with ecx = `0x012DD2B0 + i*0xA0`; then calls the shared
/// cdecl/1 callee with one fixed address and returns its result.
/// No arguments (cdecl/0). Calling convention: cdecl.
lf_checker_rt::export!(cdecl, rw_00e67080() -> u32 {
    unsafe {
        const BASE: u32 = 0x012DD2B0;
        const STRIDE: u32 = 0xA0;
        const COUNT: u32 = 5;
        const ARG: u32 = 0x00E721A0;
        let mut i = 0u32;
        while i < COUNT {
            let this = lf_checker_rt::relocated(BASE.wrapping_add(i.wrapping_mul(STRIDE)));
            lf_checker_rt::callee_thiscall!(1, u32, this);
            lf_checker_rt::callee_thiscall!(2, u32, this);
            i += 1;
        }
        lf_checker_rt::callee_cdecl!(3, u32, lf_checker_rt::relocated(ARG))
    }
});
