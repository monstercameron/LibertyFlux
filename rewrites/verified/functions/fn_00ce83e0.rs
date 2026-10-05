// original: 0x00CE83E0 CTaskSimpleNMExplosion::vf26 (merged symbol, vtable slot 26)

/// Explosion-task per-frame update: pick one of three NaturalMotion message
/// programs from flag bytes and die rolls, run it, and record the outcome.
///
/// `this` is the task, `ped` the ped. Three flag bytes drive the choice:
/// `bh` is set when the third flag is clear and (the first flag is set or a
/// die roll is below `0x3fff`); `bl` likewise with the second flag. Each die
/// is rolled only when its flag test needs it.
///
/// The task records three words: `+0x48` (a further roll below `0x3fff`),
/// `+0x44` (a scaled range pick between two globals:
/// `trunc((roll & 0xffff) * K * (G1 - G0)) + G0` with `K` a file constant),
/// and `+0x40` (2, 1 or 0 for the three paths below).
///
/// Message programs (every setter takes a name word and a value; `bufA`/`bufB`
/// are two stack message buffers, sends go through the ped's context at
/// `+0x7b4`):
/// - `bh` set: two messages. `bufA` gets a boolean, a die-roll boolean, four
///   floats from the parameter block, two booleans and five more floats with
///   a trailing zero boolean; `bufB` gets a boolean, a string slot and five
///   floats. Both are sent, the outcome word is 2, both messages destroyed.
/// - `bh` clear, `bl` set: one message. `bufA` gets a boolean, fourteen
///   floats (three share one source word), a string slot and a zero boolean.
///   Sent once, outcome 1, destroyed.
/// - Both clear: two messages. `bufB` is built first (boolean, five floats,
///   boolean), then `bufA` (boolean, die-roll boolean, one float, two
///   booleans, four floats, zero boolean). `bufB` is sent first, outcome 0,
///   both destroyed.
///
/// The only float arithmetic is the `+0x44` range pick, in the original's
/// operation order, with truncation toward zero. Returns the last callee's
/// answer. Original: 0x00CE83E0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ce83e0(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_OUTCOME: u32 = 0x40;
        const TASK_RANGE: u32 = 0x44;
        const TASK_ROLL: u32 = 0x48;
        const PED_NMCTX: u32 = 0x7b4;
        const FLAG_A: u32 = 0x171d778;
        const FLAG_B: u32 = 0x171d779;
        const FLAG_C: u32 = 0x171d77a;
        const RANGE_LO: u32 = 0x171cff4;
        const RANGE_HI: u32 = 0x171cff8;
        const RANGE_K: u32 = 0xfe8680;
        const NAME_ENABLE: u32 = 0x1051cc8;
        const ROLL_SPLIT: i32 = 0x3fff;
        const NM_CTOR: u32 = 1;
        const NM_SETBOOL: u32 = 2;
        const NM_SETFLOAT: u32 = 3;
        const NM_SETSTR: u32 = 4;
        const NM_SEND: u32 = 5;
        const NM_DTOR: u32 = 6;
        const RAND: u32 = 7;
        const COOKIE: u32 = 8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn gb(va: u32) -> u8 {
            unsafe { (lf_checker_rt::relocated(va) as *const u8).read() }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        /// Truncating float-to-int exactly like `cvttss2si`: out-of-range
        /// and NaN give `0x80000000`, otherwise round toward zero.
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
                0x80000000u32 as i32
            } else {
                x as i32
            }
        }

        let mut eax = 0u32;

        // Path flags; each die is rolled only when its flag is clear.
        let bh = if gb(FLAG_A) != 0 {
            gb(FLAG_C) == 0
        } else {
            let r = lf_checker_rt::callee_cdecl!(RAND, u32,);
            eax = r;
            (r as i32) < ROLL_SPLIT && gb(FLAG_C) == 0
        };
        let bl = if gb(FLAG_B) != 0 {
            gb(FLAG_C) == 0
        } else {
            let r = lf_checker_rt::callee_cdecl!(RAND, u32,);
            eax = r;
            (r as i32) < ROLL_SPLIT && gb(FLAG_C) == 0
        };
        let r3 = lf_checker_rt::callee_cdecl!(RAND, u32,);
        eax = r3;
        wr8(this + TASK_ROLL, if (r3 as i32) < ROLL_SPLIT { 1 } else { 0 });
        let glo = g32(RANGE_LO);
        let ghi = g32(RANGE_HI);
        let r4 = lf_checker_rt::callee_cdecl!(RAND, u32,);
        eax = r4;
        let span = ghi.wrapping_sub(glo);
        let t = mul(
            core::hint::black_box((r4 & 0xffff) as i32) as f32,
            f32::from_bits(g32(RANGE_K)),
        );
        let x = mul(t, core::hint::black_box(span as i32) as f32);
        wr32(this + TASK_RANGE, (cvtt(x) as u32).wrapping_add(glo));

        let mut msg_a = [0u32; 16];
        let buf_a = msg_a.as_mut_ptr() as u32;
        let mut msg_b = [0u32; 16];
        let buf_b = msg_b.as_mut_ptr() as u32;
        let nmctx = rd32(ped + PED_NMCTX);

        if bh {
            eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf_a);
            eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf_b);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(NAME_ENABLE), 1);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_b, g32(NAME_ENABLE), 1);
            let r5 = lf_checker_rt::callee_cdecl!(RAND, u32,);
            eax = r5;
            let rb = if (r5 as i32) < ROLL_SPLIT { 1 } else { 0 };
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(0x1051dd4), rb);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_a, g32(0x1051dd8), g32(0x171cf9c));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(0x1051ddc), 1);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(0x1051de0), 1);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_a, g32(0x1051de4), g32(0x171cfa0));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_a, g32(0x1051de8), g32(0x171cfa4));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_a, g32(0x1051dec), g32(0x171cfa8));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_a, g32(0x1051df0), g32(0x171cfac));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(0x1051df4), 0);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETSTR, u32, buf_b, g32(0x1051d4c),
                lf_checker_rt::relocated(0xedcf6c));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_b, g32(0x1051d50), g32(0x171cfe0));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_b, g32(0x1051d54), g32(0x171cfe4));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_b, g32(0x1051d58), g32(0x171cfe8));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_b, g32(0x1051d5c), g32(0x171cfec));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_b, g32(0x1051d60), g32(0x171cff0));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SEND, u32, nmctx, g32(0x1051dcc), buf_a);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SEND, u32, nmctx, g32(0x1051d44), buf_b);
            wr32(this + TASK_OUTCOME, 2);
            eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf_b);
            eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf_a);
        } else if bl {
            eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf_a);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(NAME_ENABLE), 1);
            const BL_FLOATS: [(u32, u32); 14] = [
                (0x1051d6c, 0x171cfb0),
                (0x1051d70, 0x171cfb4),
                (0x1051d74, 0x171cfb8),
                (0x1051d78, 0x171cfbc),
                (0x1051d7c, 0x171cfc0),
                (0x1051d80, 0x171cfc4),
                (0x1051d84, 0x171cfc8),
                (0x1051d88, 0x171cfc8),
                (0x1051d8c, 0x171cfc8),
                (0x1051d90, 0x171cfcc),
                (0x1051d94, 0x171cfd0),
                (0x1051d98, 0x171cfd4),
                (0x1051d9c, 0x171cfd8),
                (0x1051da0, 0x171cfdc),
            ];
            for (name, src) in BL_FLOATS {
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SETFLOAT, u32, buf_a, g32(name), g32(src));
            }
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETSTR, u32, buf_a, g32(0x1051da4),
                lf_checker_rt::relocated(0xedcf70));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(0x1051da8), 0);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SEND, u32, nmctx, g32(0x1051d64), buf_a);
            wr32(this + TASK_OUTCOME, 1);
            eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf_a);
        } else {
            eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf_b);
            eax = lf_checker_rt::callee_thiscall!(NM_CTOR, u32, buf_a);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_b, g32(NAME_ENABLE), 1);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(NAME_ENABLE), 1);
            const FT_FLOATS_B: [(u32, u32); 5] = [
                (0x1051db4, 0x171cf88),
                (0x1051db8, 0x171cf8c),
                (0x1051dbc, 0x171cf90),
                (0x1051dc0, 0x171cf94),
                (0x1051dc4, 0x171cf98),
            ];
            for (name, src) in FT_FLOATS_B {
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SETFLOAT, u32, buf_b, g32(name), g32(src));
            }
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_b, g32(0x1051dc8), 1);
            let r5 = lf_checker_rt::callee_cdecl!(RAND, u32,);
            eax = r5;
            let rb = if (r5 as i32) < ROLL_SPLIT { 1 } else { 0 };
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(0x1051dd4), rb);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETFLOAT, u32, buf_a, g32(0x1051dd8), g32(0x171cf9c));
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(0x1051ddc), 1);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(0x1051de0), 1);
            const FT_FLOATS_A: [(u32, u32); 4] = [
                (0x1051de4, 0x171cfa0),
                (0x1051de8, 0x171cfa4),
                (0x1051dec, 0x171cfa8),
                (0x1051df0, 0x171cfac),
            ];
            for (name, src) in FT_FLOATS_A {
                eax = lf_checker_rt::callee_thiscall!(
                    NM_SETFLOAT, u32, buf_a, g32(name), g32(src));
            }
            eax = lf_checker_rt::callee_thiscall!(
                NM_SETBOOL, u32, buf_a, g32(0x1051df4), 0);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SEND, u32, nmctx, g32(0x1051dac), buf_b);
            eax = lf_checker_rt::callee_thiscall!(
                NM_SEND, u32, nmctx, g32(0x1051dcc), buf_a);
            wr32(this + TASK_OUTCOME, 0);
            eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf_a);
            eax = lf_checker_rt::callee_thiscall!(NM_DTOR, u32, buf_b);
        }
        lf_checker_rt::callee_stdcall!(COOKIE, u32,);
        eax
    }
});
