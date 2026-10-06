// original: 0x008F6B80 Input_ReleaseAll

/// Release the four input objects in order, run the shared teardown on the
/// default object, then notify with argument 1. Returns the notification's
/// answer. All object addresses are relocated immediates. Convention: cdecl,
/// no stack words.
lf_checker_rt::export!(cdecl, rw_008f6b80() -> u32 {
    unsafe {
        const RELEASE: u32 = 1;
        const TEARDOWN: u32 = 2;
        const NOTIFY: u32 = 3;
        const OBJ_A: u32 = 0x0117E700;
        const OBJ_B: u32 = 0x01182184;
        const OBJ_C: u32 = 0x01185C08;
        const OBJ_D: u32 = 0x0118968C;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            RELEASE,
            u32,
            lf_checker_rt::relocated(OBJ_A)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            RELEASE,
            u32,
            lf_checker_rt::relocated(OBJ_B)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            RELEASE,
            u32,
            lf_checker_rt::relocated(OBJ_C)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            RELEASE,
            u32,
            lf_checker_rt::relocated(OBJ_D)
        );
        let _: u32 = lf_checker_rt::callee_thiscall!(
            TEARDOWN,
            u32,
            lf_checker_rt::relocated(OBJ_C)
        );
        lf_checker_rt::callee_cdecl!(NOTIFY, u32, 1)
    }
});
