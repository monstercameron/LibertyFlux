// original: 0x00cb1f80 task_stand_gate (proposed)

/// Gate a stand-still subtask on a random draw, a validation call and a
/// fetched behaviour object.
///
/// `this` is the complex task: anchor vector at `+0x30`, state word at
/// `+0x74`. `arg` is the candidate description passed to the validator.
/// Callee 1 is the landed random-number helper (any 32-bit answer is folded
/// the same way); callee 2 validates `arg` (only its low byte matters);
/// callee 3 fetches the behaviour object for the global at `0x167e2a0`;
/// callee 4 builds the subtask.
///
/// Behaviour: draws `r` from callee 1 and scales it to a speed factor: with
/// state bit 2 set the factor is `((r * K1) * 0.75 + 0.75) + 1.0`, otherwise
/// `((r * K1) * 0.5) + 1.0`, where `K1` is the fixed scale `0x38000100`.
/// When the validator accepts `arg`, the factor is raised to at least 1.5.
/// Callee 3 then fetches the object for the global: a null answer takes the
/// fault path (a bit is set through the null pointer, faulting exactly like
/// the original, before the state bit below is set). Otherwise callee 4 is
/// ordered with (factor, anchor, 0.5, 3.0, -1, 1, 0, 0, 0, 1) and the object
/// it returns gets bit `0x2000` set at `+0xd8`. State bit 0 is set and
/// callee 4's answer is returned.
///
/// Original: 0x00cb1f80 (thiscall, one stack word; callee 1 is cdecl with no
/// arguments, callee 2 is cdecl with one, callee 3 is thiscall with none and
/// takes the global in ECX, callee 4 is thiscall with ten stack words and
/// takes callee 3's answer in ECX).
lf_checker_rt::export!(thiscall, rw_00cb1f80(this: u32, arg: u32) -> u32 {
    unsafe {
        const ANCHOR: u32 = 0x30;
        const STATE: u32 = 0x74;
        const K1: f32 = f32::from_bits(0x3800_0100);
        const C75: f32 = 0.75;
        const C50: f32 = 0.5;
        const C10: f32 = 1.0;
        const C15: f32 = 1.5;
        const C30: f32 = 3.0;
        const C05: f32 = 0.5;
        const GLOBAL_STATE: u32 = 0x167e2a0;
        const OBJ_BIT_OFF: u32 = 0xd8;
        const OBJ_BIT: u32 = 0x2000;
        const CALLEE_RAND: u32 = 1;
        const CALLEE_VALID: u32 = 2;
        const CALLEE_FETCH: u32 = 3;
        const CALLEE_BUILD: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let r = lf_checker_rt::callee_cdecl!(CALLEE_RAND, u32,) as i32 as f32;
        let mut f = if rd8(this + STATE) & 4 != 0 {
            add(add(mul(mul(r, K1), C75), C75), C10)
        } else {
            add(mul(mul(r, K1), C50), C10)
        };
        let ok = lf_checker_rt::callee_cdecl!(CALLEE_VALID, u32, arg);
        if ok & 0xff != 0 && !(f > C15) {
            f = C15;
        }
        let g = (lf_checker_rt::global::<u32>(GLOBAL_STATE) as *const u32).read_unaligned();
        let c = lf_checker_rt::callee_thiscall!(CALLEE_FETCH, u32, g);
        if c == 0 {
            let p = OBJ_BIT_OFF as *mut u32;
            p.write_unaligned(p.read_unaligned() | OBJ_BIT);
            wr32(this + STATE, rd32(this + STATE) | 1);
            return 0;
        }
        let d = lf_checker_rt::callee_thiscall!(
            CALLEE_BUILD, u32, c, f.to_bits(), this.wrapping_add(ANCHOR),
            C05.to_bits(), C30.to_bits(), 0xffff_ffff, 1, 0, 0, 0, 1
        );
        let q = (d.wrapping_add(OBJ_BIT_OFF)) as *mut u32;
        q.write_unaligned(q.read_unaligned() | OBJ_BIT);
        wr32(this + STATE, rd32(this + STATE) | 1);
        d
    }
});
