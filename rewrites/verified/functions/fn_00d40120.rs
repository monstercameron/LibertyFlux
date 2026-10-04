// original: 0x00d40120 CTaskSimpleJumpLaunch::vf20

/// Decide whether a jump-launch task may start (slot vf20).
///
/// `this` is the task, `owner` (arg0) the ped. The dword at `LINK`
/// (+0x10) must point to a block whose dword at +0x74 has bit 14 set,
/// else 0. When the owner flag bytes at `G_A` (+0x218) and `G_B` (+0x219)
/// are 0 and nonzero respectively, the main path runs: the find callee
/// (thiscall on `owner`) must return non-null, then the first getter
/// (thiscall on that) feeds the test callee (cdecl, one word); a nonzero
/// test returns 1, else the second getter feeds the test callee again and
/// its zero/nonzero decides 0/1. Any other flag combination takes the
/// still path: the squared 2D speed from the block at `SPEED` (+0xa80)
/// (+0xc squared plus +0x10 squared, y first) returns 1 only when
/// strictly above 0.01 (ordered). Only al carries the result.
///
/// Original: 0x00d40120 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00d40120(this: u32, owner: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x10;
        const G_A: u32 = 0x218;
        const G_B: u32 = 0x219;
        const SPEED: u32 = 0xa80;
        const C_MIN2: u32 = 0x00fe_870c;
        const FIND: u32 = 1;
        const GET_A: u32 = 2;
        const TEST: u32 = 3;
        const GET_B: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let link = rd32(this + LINK);
        if link == 0 || rd32(link + 0x74) >> 14 & 1 == 0 {
            return 0;
        }
        if ((owner + G_A) as *const u8).read() == 0
            && ((owner + G_B) as *const u8).read() != 0
        {
            let found: u32 = lf_checker_rt::callee_thiscall!(FIND, u32, owner);
            if found == 0 {
                return 0;
            }
            let got: u32 = lf_checker_rt::callee_thiscall!(GET_A, u32, found);
            let tested: u32 = lf_checker_rt::callee_cdecl!(TEST, u32, got);
            if tested != 0 {
                return 1;
            }
            let got2: u32 = lf_checker_rt::callee_thiscall!(GET_B, u32, found);
            let tested2: u32 = lf_checker_rt::callee_cdecl!(TEST, u32, got2);
            if tested2 != 0 { 1 } else { 0 }
        } else {
            let sp = rd32(owner + SPEED);
            let vy = rdf(sp + 0x0c);
            let vx = rdf(sp + 0x10);
            let sum = add(mul(vy, vy), mul(vx, vx));
            let min = f32::from_bits(rd32(lf_checker_rt::relocated(C_MIN2)));
            if sum > min { 1 } else { 0 }
        }
    }
});
