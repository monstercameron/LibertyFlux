// original: 0x00c182f0 clamped_helper_diff_stored_byte

/// Clamp a helper value minus the argument bits, and store the byte.
///
/// Returns at once when either argument is null. Otherwise loads a per-index
/// object from the global table at `0x01295cd8` (indexed by the SIGNED word
/// at `a1+0x2e`), calls its virtual slot `+0x38` with argument 7, then calls
/// the fetch helper with `a1` and that answer, and forms `fetch+0x38` minus
/// the scaled float (`inner+0x38` plus 0.6, which the original spills over
/// its own argument slot and re-reads). Values strictly below -2.0 clamp
/// to -2.0 and values strictly above 1.0 clamp to 1.0 (both jumps are `ja`,
/// which never takes the unordered path, so NaN passes through unclamped);
/// the result times 50 is truncated with `cvttss2si` semantics (NaN gives
/// `0x80000000`, whose low byte is 0) and its low byte stored at `a0+0xa`.
/// The rewrite holds the spilled value in a variable instead of
/// round-tripping it through the stack, so the contract switches the stack
/// check off; the early paths return the unread entry residue, so the return
/// value is not compared and the byte in the heap carries the proof.
///
/// Original: 0x00C182F0 (stdcall, two stack words).
lf_checker_rt::export!(stdcall, rw_00c182f0(a0: u32, a1: u32) -> u32 {
    unsafe {
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            let r = core::hint::black_box(a) * core::hint::black_box(b);
            core::hint::black_box(a);
            core::hint::black_box(b);
            r
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            let r = core::hint::black_box(a) + core::hint::black_box(b);
            core::hint::black_box(a);
            core::hint::black_box(b);
            r
        }
        const FETCH: u32 = 2;
        const TABLE: u32 = 0x0129_5cd8;
        const VTABLE_SLOT: u32 = 0x38;
        if a1 == 0 {
            return 0;
        }
        if a0 == 0 {
            return 0;
        }
        let inner = ((a1 + 0x20) as *const u32).read_unaligned();
        let base = f32::from_bits(((inner + 0x38) as *const u32).read_unaligned());
        let scaled = fadd(base, lf_checker_rt::global::<f32>(0x00fe_8858).read());
        let idx = ((a1 + 0x2e) as *const i16).read_unaligned() as i32;
        let entry = lf_checker_rt::relocated(TABLE).wrapping_add((idx as u32).wrapping_mul(4));
        let obj = (entry as *const u32).read_unaligned();
        let vtable = (obj as *const u32).read_unaligned();
        let slot = ((vtable + VTABLE_SLOT) as *const u32).read_unaligned() as usize;
        let pick: extern "thiscall" fn(u32, u32) -> u32 = core::mem::transmute(slot);
        let picked: u32 = pick(obj, 7);
        let r: u32 = lf_checker_rt::callee_thiscall!(FETCH, u32, a1, picked);
        let p = f32::from_bits(((r + 0x38) as *const u32).read_unaligned());
        let x = p - scaled;
        let lo = lf_checker_rt::global::<f32>(0x00fe_8db0).read();
        let hi = lf_checker_rt::global::<f32>(0x00fe_88e8).read();
        let v = if lo > x {
            lo
        } else if x > hi {
            hi
        } else {
            x
        };
        let s = fmul(v, lf_checker_rt::global::<f32>(0x00fe_8b68).read());
        let t = if s.is_nan() { 0x8000_0000u32 as i32 } else { s as i32 };
        ((a0 + 0xa) as *mut u8).write(t as u8);
        t as u32
    }
});
