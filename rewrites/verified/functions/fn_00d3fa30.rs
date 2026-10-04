// original: 0x00d3fa30 jump_pick_land_anims (proposed)

/// Pick the land-animation ids for a jump from height and sideways speed.
///
/// `this` is the task, holding the vertical speed at `VEL_Z` (+0x38), the
/// horizontal speeds at `VEL_X` (+0x30) / `VEL_Y` (+0x34) and a flag byte at
/// `TASK_FLAGS` (+0x70). `info` (arg0) links two helper blocks at `INFO_A`
/// (+0xab0) and `INFO_B` (+0xa80); `primary` (arg1) and `secondary` (arg2)
/// are out-words, preset to 9 and 0x7f.
///
/// Selection, in order: when `INFO_A` is present, its masked mode
/// ([+0x28] & 0x3c0) equals 0x80 and its state ([+0x1304]) equals 2, the
/// presets are kept. Otherwise, when the height threshold (-4.3) is not
/// above `VEL_Z`, or the squared horizontal speed is not above 1.0, the
/// slow path runs: the presets are kept only when both words at `INFO_B`
/// (+0xc, +0x10) read +0.0/-0.0 (ordered equal, so NaN falls through);
/// otherwise `primary` becomes 1 and `secondary` 0x57 or 0x58 from bit 1
/// of `TASK_FLAGS`. When both thresholds are passed, `primary` becomes 1
/// and `secondary` is 0x5a unless the second height threshold (-5.9) is
/// strictly above `VEL_Z` (ordered), in which case 0x59. All float
/// comparisons treat unordered (NaN) as not-above; arithmetic order is the
/// original's (x*x, y*y, then add).
///
/// No return value (eax keeps a scratch value), no calls, no globals; the
/// three threshold constants are read from the image.
///
/// Original: 0x00d3fa30 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00d3fa30(this: u32, info: u32, primary: u32, secondary: u32) -> u32 {
    unsafe {
        const INFO_A: u32 = 0xab0;
        const INFO_B: u32 = 0xa80;
        const VEL_X: u32 = 0x30;
        const VEL_Y: u32 = 0x34;
        const VEL_Z: u32 = 0x38;
        const TASK_FLAGS: u32 = 0x70;
        const MODE_MASK: u32 = 0x3c0;
        const MODE_WANT: u32 = 0x80;
        const C_Z: u32 = 0x00e9_81b4;
        const C_SPEED2: u32 = 0x00fe_88e8;
        const C_Z2: u32 = 0x00ee_3ef4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn fconst(file_va: u32) -> f32 {
            unsafe { f32::from_bits(rd32(lf_checker_rt::relocated(file_va))) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        (primary as *mut u32).write_unaligned(9);
        (secondary as *mut u32).write_unaligned(0x7f);
        let helper = rd32(info + INFO_A);
        let keep = helper != 0
            && rd32(helper + 0x28) & MODE_MASK == MODE_WANT
            && rd32(helper + 0x1304) == 2;
        if !keep {
            let vz = rdf(this + VEL_Z);
            let slow = if fconst(C_Z) > vz {
                let speed2 = add(mul(rdf(this + VEL_X), rdf(this + VEL_X)), mul(rdf(this + VEL_Y), rdf(this + VEL_Y)));
                !(speed2 > fconst(C_SPEED2))
            } else {
                true
            };
            if slow {
                let still = rd32(info + INFO_B);
                let px = rdf(still + 0x0c);
                let py = rdf(still + 0x10);
                if !(px == 0.0 && py == 0.0) {
                    (primary as *mut u32).write_unaligned(1);
                    let flag2 = unsafe { ((this + TASK_FLAGS) as *const u8).read() } & 2 != 0;
                    let pick = if flag2 { 0x57 } else { 0x58 };
                    (secondary as *mut u32).write_unaligned(pick);
                }
            } else {
                (primary as *mut u32).write_unaligned(1);
                let pick = if fconst(C_Z2) > vz { 0x59 } else { 0x5a };
                (secondary as *mut u32).write_unaligned(pick);
            }
        }
        0
    }
});
