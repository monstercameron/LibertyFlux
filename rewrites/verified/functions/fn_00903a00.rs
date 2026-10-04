// original: 0x00903A00 ui_setup_const (proposed)
//
// Twin of 0x009038A0 with a constant first path: mode 0 passes fixed words
// (two halves, two copies of a mid-range constant, a marker slot) to the
// five-word setup callee, mode 1 is the same five-pointer call as its twin,
// any other mode skips both. Then the same four-call poke tail and cookie
// check. Takes no arguments. Returns the tail's last answer.
//
// Original: 0x00903A00 (cdecl, no stack words).
lf_checker_rt::export!(cdecl, rw_00903A00() -> u32 {
    unsafe {
        const SETUP5: u32 = 1;
        const SETUPW: u32 = 2;
        const HALF: u32 = 0x3F00_0000;
        const MID: u32 = 0x3F17_0A3E;
        const MODE: u32 = 0x0103_44B4;
        let mode = (lf_checker_rt::global::<u32>(MODE) as *const u32).read_unaligned();
        if mode == 0 {
            let mut marker: u32 = 0xFFFF_FFFF;
            let _: u32 = lf_checker_rt::callee_cdecl!(
                SETUPW, u32, HALF, HALF, MID, MID,
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
