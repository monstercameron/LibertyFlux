// original: 0x009038A0 ui_setup_dual_float (proposed)
//
// Configure the input layer from a mode global and two float arguments.
//
// Reads the mode word: mode 0 shifts both arguments by a constant pair taken
// from read-only data (`arg - 0.5 + ~0.09`, argument 1 first) and passes them
// with two halves and a marker slot to the five-word setup callee; mode 1
// passes five pointers to constant slots (`1.0, 1.0, 0, 0, 0xFFFFFFFF`) to
// the five-pointer setup callee; any other mode skips both. Then runs the
// four-call poke tail and the register-preserving cookie check. Returns the
// tail's last answer.
//
// Original: 0x009038A0 (cdecl, two stack words holding float bits).
lf_checker_rt::export!(cdecl, rw_009038A0(a0: u32, a1: u32) -> u32 {
    unsafe {
        const SETUP5: u32 = 1;
        const SETUPW: u32 = 2;
        const HALF: u32 = 0x3F00_0000;
        const MODE: u32 = 0x0103_44B4;
        const SUB_C: u32 = 0x00FE_8830;
        const ADD_C: u32 = 0x00FE_8794;
        let mode = (lf_checker_rt::global::<u32>(MODE) as *const u32).read_unaligned();
        if mode == 0 {
            let sub = f32::from_bits((lf_checker_rt::global::<u32>(SUB_C) as *const u32).read_unaligned());
            let add = f32::from_bits((lf_checker_rt::global::<u32>(ADD_C) as *const u32).read_unaligned());
            let f1 = fadd(fsub(f32::from_bits(a1), sub), add);
            let f0 = fadd(fsub(f32::from_bits(a0), sub), add);
            let mut marker: u32 = 0xFFFF_FFFF;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                SETUPW, u32, HALF, HALF, f0.to_bits(), f1.to_bits(),
                &mut marker as *mut u32 as u32
            );
        } else if mode == 1 {
            let mut s0: u32 = 0x3F80_0000;
            let mut s1: u32 = 0x3F80_0000;
            let mut s2: u32 = 0;
            let mut s3: u32 = 0;
            let mut s4: u32 = 0xFFFF_FFFF;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                SETUP5, u32,
                &mut s0 as *mut u32 as u32, &mut s1 as *mut u32 as u32,
                &mut s2 as *mut u32 as u32, &mut s3 as *mut u32 as u32,
                &mut s4 as *mut u32 as u32
            );
        }
        setup_tail()
    }
});

// Twin setup helpers shared by 0x009038A0 and 0x00903A00.
#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}
#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

// Run the four-call tail shared by both twins; returns the last answer.
unsafe fn setup_tail() -> u32 {
    unsafe {
        const POKE: u32 = 3;
        const COOKIE_CHECK: u32 = 4;
        let mut ans: u32 = lf_checker_rt::callee_cdecl!(POKE, u32, 0x0A, 0);
        ans = lf_checker_rt::callee_cdecl!(POKE, u32, 2, 5);
        ans = lf_checker_rt::callee_cdecl!(POKE, u32, 0x0F, 8);
        ans = lf_checker_rt::callee_cdecl!(POKE, u32, 7, 0);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,);
        ans
    }
}
