// original: 0x00a0b420 cleanup_validated_reinit (proposed)
/// Re-initialise both cleanup tables, validating every record first.
///
/// Builds an empty record on the stack, then for each of the 0x100 records
/// at `this + 4` validates it strictly: live records are inspected as-is,
/// dead ones are inspected via the empty record. Same for the 0xc8 records
/// at `this + 0x2c04` with the flag clear. (Inspect/validate/create are
/// stubs under the checker, so the proof is the call sequence plus the
/// snapshot of each inspected record.) The stack-cookie check runs as a
/// final no-argument call; its esp-derived register is not compared.
/// Always answers 0. Thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00a0b420(this: u32) -> u32 {
    unsafe {
        const EMPTY: u32 = 0;
        const VALIDATE: u32 = 1;
        const INSPECT: u32 = 2;
        const COOKIE_CHECK: u32 = 3;
        const COUNT_A: u32 = 0x100;
        const COUNT_B: u32 = 0xc8;
        const STRIDE: u32 = 0x2c;
        const TABLE_B: u32 = 0x2c04;
        let buf = [0u32; 11];
        lf_checker_rt::callee_thiscall!(EMPTY, u32, buf.as_ptr() as u32);
        let mut rec = this + 4;
        let mut i = 0u32;
        while i < COUNT_A {
            let ok: u32 = lf_checker_rt::callee_thiscall!(VALIDATE, u32, this, rec, 1u32);
            let src = if (ok & 0xff) != 0 { rec } else { buf.as_ptr() as u32 };
            let _: u32 = lf_checker_rt::callee_cdecl!(INSPECT, u32, src, STRIDE);
            i += 1;
            rec += STRIDE;
        }
        let mut rec = this + TABLE_B;
        let mut i = 0u32;
        while i < COUNT_B {
            let ok: u32 = lf_checker_rt::callee_thiscall!(VALIDATE, u32, this, rec, 0u32);
            let src = if (ok & 0xff) != 0 { rec } else { buf.as_ptr() as u32 };
            let _: u32 = lf_checker_rt::callee_cdecl!(INSPECT, u32, src, STRIDE);
            i += 1;
            rec += STRIDE;
        }
        lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,)
    }
});
