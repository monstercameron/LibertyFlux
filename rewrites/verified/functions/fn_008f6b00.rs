// original: 0x008F6B00 Input_GetObject

/// Return the shared object: when the mode register holds -1 the ensure
/// step runs first with argument 0, then the selector is called and its
/// result passed (via ECX) to the shared teardown in tail position, whose
/// answer is returned. Convention: cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_008f6b00() -> u32 {
    unsafe {
        const ENSURE: u32 = 1;
        const SELECT: u32 = 2;
        const TEARDOWN: u32 = 3;
        const MODE_REG: u32 = 0x010330F8;
        if lf_checker_rt::global::<u32>(MODE_REG).read_unaligned() == 0xFFFF_FFFF {
            let _: u32 = lf_checker_rt::callee_cdecl!(ENSURE, u32, 0);
        }
        let o: u32 = lf_checker_rt::callee_cdecl!(SELECT, u32,);
        lf_checker_rt::callee_thiscall!(TEARDOWN, u32, o)
    }
});
