// original: 0x00a0a420 mission_cleanup_reinit (proposed)
/// Re-initialise both mission-cleanup tables.
///
/// Clears all 0x100 records of 0x2c bytes at `this + 4` and re-registers
/// each unless the global quiet flag is set, then clears the 0xc8 records
/// at `this + 0x2c04`. (The clear and register callees are stubs under the
/// checker, so the proof is the call sequence.) Thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00a0a420(this: u32) -> u32 {
    unsafe {
        const QUIET: u32 = 0x0116d27d;
        const CLEAR: u32 = 0;
        const REGISTER: u32 = 1;
        const COUNT_A: u32 = 0x100;
        const COUNT_B: u32 = 0xc8;
        const STRIDE: u32 = 0x2c;
        const TABLE_B: u32 = 0x2c04;
        let quiet = (lf_checker_rt::global::<u8>(QUIET) as *const u8).read();
        let mut rec = this + 4;
        let mut i = 0u32;
        while i < COUNT_A {
            let _: u32 = lf_checker_rt::callee_cdecl!(CLEAR, u32, rec, STRIDE);
            if quiet == 0 {
                let _: u32 = lf_checker_rt::callee_thiscall!(REGISTER, u32, this, rec);
            }
            i += 1;
            rec += STRIDE;
        }
        let mut rec = this + TABLE_B;
        let mut i = 0u32;
        while i < COUNT_B {
            let _: u32 = lf_checker_rt::callee_cdecl!(CLEAR, u32, rec, STRIDE);
            i += 1;
            rec += STRIDE;
        }
        0
    }
});
