// original: 0x00e5c1b0 init_then_register
/// Runs the subsystem initializer, then registers tag 0x00e6e300.
/// The entry ECX value is forwarded to the initializer as its
/// argument. Returns the registrar's answer.
export!(thiscall, rw_00e5c1b0(this_ecx: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x00E6E300;
        callee_stdcall!(1, u32, this_ecx);
        callee_cdecl!(2, u32, relocated(TAG))
    }
});
