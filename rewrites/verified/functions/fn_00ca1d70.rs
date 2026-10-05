// original: 0x00CA1D70 ped_task_block_init (proposed)

/// Initialise a ped task block: construct three sub-objects, clear the
/// parameter area, derive a countdown from a random draw and run two setup steps.
///
/// `this` points to a block of at least 0x184 bytes. Three embedded
/// sub-objects are constructed in place (at `+0x00`, `+0x80` and `+0x110`).
/// The parameter area `+0x12C..+0x156` is then cleared except for a
/// unity scale at `+0x144`; the control words at `+0x170`/`+0x178`,
/// the flag byte at `+0x17C` and the tail word at `+0x180` are cleared
/// while the selector at `+0x174` is set to -1 (none).
///
/// A fourth construction step runs at `this`, then a random draw is taken
/// modulo 65536, converted to float, scaled by two constant factors
/// (2^-15, then -500.0) and truncated to an integer; 500 minus that value
/// is stored at `+0x150`, so the stored countdown lies in 500..=1499.
/// The shared tick counter global is copied to `+0x14C` and the ready
/// flag at `+0x154` is set. Finally two setup calls run (each with a
/// zero argument), the result slot at `+0x158` is cleared, and `this`
/// is returned.
///
/// Original: 0x00CA1D70 (thiscall, no stack arguments, returns `this`).
/// Callees 0-6 are the seven direct calls in order; callee 4 answers the
/// random draw. Float order is the original's: (n * 2^-15) * -500.0.
lf_checker_rt::export!(thiscall, rw_00ca1d70(this: u32) -> u32 {
    unsafe {
        const SUB_B: u32 = 0x80;
        const SUB_C: u32 = 0x110;
        const PARAM_LO: u32 = 0x12c;
        const PARAM_WORDS: u32 = 6;
        const SCALE_ONE: u32 = 0x144;
        const TICK_COPY: u32 = 0x14c;
        const COUNTDOWN: u32 = 0x150;
        const READY_FLAG: u32 = 0x154;
        const CTRL_A: u32 = 0x170;
        const SELECTOR: u32 = 0x174;
        const CTRL_B: u32 = 0x178;
        const FLAG_B: u32 = 0x17c;
        const TAIL: u32 = 0x180;
        const RESULT: u32 = 0x158;
        const COUNTDOWN_BASE: u32 = 500;
        const TICK_GLOBAL: u32 = 0x0117_35b4;
        const INV_2P15: u32 = 0x00fe_8680;
        const NEG_500: u32 = 0x00ea_ebe4;
        const CALLEE_CTOR_A: u32 = 0;
        const CALLEE_CTOR_B: u32 = 1;
        const CALLEE_CTOR_C: u32 = 2;
        const CALLEE_SETUP0: u32 = 3;
        const CALLEE_DRAW: u32 = 4;
        const CALLEE_SETUP1: u32 = 5;
        const CALLEE_SETUP2: u32 = 6;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CTOR_A, u32, this);
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CTOR_B, u32, this.wrapping_add(SUB_B));
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_CTOR_C, u32, this.wrapping_add(SUB_C));
        for i in 0..PARAM_WORDS {
            wr32(this.wrapping_add(PARAM_LO).wrapping_add(i * 4), 0);
        }
        wr32(this.wrapping_add(SCALE_ONE), 1.0f32.to_bits());
        wr32(this.wrapping_add(SCALE_ONE).wrapping_add(4), 0);
        wr32(this.wrapping_add(TICK_COPY), 0);
        wr32(this.wrapping_add(COUNTDOWN), 0);
        ((this.wrapping_add(READY_FLAG)) as *mut u16).write_unaligned(0);
        wr32(this.wrapping_add(CTRL_A), 0);
        wr32(this.wrapping_add(SELECTOR), 0xffff_ffff);
        wr32(this.wrapping_add(CTRL_B), 0);
        ((this.wrapping_add(FLAG_B)) as *mut u8).write(0);
        wr32(this.wrapping_add(TAIL), 0);

        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_SETUP0, u32, this);
        let draw: u32 = lf_checker_rt::callee_cdecl!(CALLEE_DRAW, u32,);
        let tick: u32 = lf_checker_rt::global::<u32>(TICK_GLOBAL).read();
        wr32(this.wrapping_add(TICK_COPY), tick);
        let c1 = f32::from_bits(lf_checker_rt::global::<u32>(INV_2P15).read());
        let c2 = f32::from_bits(lf_checker_rt::global::<u32>(NEG_500).read());
        let scaled = mul(mul((draw & 0xffff) as f32, c1), c2);
        ((this.wrapping_add(READY_FLAG)) as *mut u8).write(1);
        let take = scaled as i32;
        wr32(
            this.wrapping_add(COUNTDOWN),
            COUNTDOWN_BASE.wrapping_sub(take as u32),
        );

        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_SETUP1, u32, this, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(CALLEE_SETUP2, u32, this, 0);
        wr32(this.wrapping_add(RESULT), 0);
        this
    }
});
