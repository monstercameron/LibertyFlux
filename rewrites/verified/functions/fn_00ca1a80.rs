// original: 0x00CA1A80 ped_task_pose_update (proposed)

/// Refresh a ped task's pose from its driver's transform, or copy it through.
///
/// `this` points to the task (source position at `+0x30`, destination at
/// `+0x40`, driver object at `+0x14`, selector at `+0x2c`, mode flags at
/// `+0x5c`). Behaviour by mode:
/// - Bit 2 clear: nothing is done.
/// - Bit 1 clear, or a null driver: the source quad (`+0x30..+0x3c`) is
///   copied to the destination (`+0x40..+0x4c`).
/// - Otherwise, when the selector is not -1 and the driver kind
///   (`[driver+0x28] & 0x3c0`) is `0xc0`, a readiness call runs on the
///   driver. If it answers nonzero, three shared offset globals are
///   gathered into a frame array, a mapping call refreshes the array, a
///   matrix call answers the driver's matrix, and the destination is the
///   array plus the matrix applied to the source. If it answers zero,
///   the destination is the matrix applied to the source plus the
///   matrix's translation column, with a zero fourth word.
/// - Otherwise (selector -1 or another driver kind): when the driver's
///   matrix slot (`+0x20`) is null it is produced by two setup calls,
///   then the destination is computed as in the previous case.
///
/// The matrix is three rows at stride 16 with the translation column at
/// `+0x30`. The fourth destination word on the zero/nonzero-answer paths
/// is the original's uninitialised frame slot, 0 under the checker's
/// defined stack fill. Float order is the original's.
///
/// Original: 0x00CA1A80 (thiscall, no stack arguments, no return value).
/// Callees: 0 readiness, 1 mapping (frame array + selector), 2 matrix,
/// 3/4 matrix-slot setup.
lf_checker_rt::export!(thiscall, rw_00ca1a80(this: u32) -> u32 {
    unsafe {
        const DRIVER: u32 = 0x14;
        const SELECTOR: u32 = 0x2c;
        const SRC: u32 = 0x30;
        const DST: u32 = 0x40;
        const FLAGS: u32 = 0x5c;
        const DRV_MATRIX: u32 = 0x20;
        const DRV_KIND: u32 = 0x28;
        const KIND_MASK: u32 = 0x3c0;
        const KIND_WANT: u32 = 0xc0;
        const NO_SELECTOR: u32 = 0xffff_ffff;
        const OFF_G0: u32 = 0x01b4_b2a0;
        const OFF_G1: u32 = 0x01b4_b2a4;
        const OFF_G2: u32 = 0x01b4_b2a8;
        const READY: u32 = 0;
        const MAPPING: u32 = 1;
        const MATRIX: u32 = 2;
        const SETUP0: u32 = 3;
        const SETUP1: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// One 3-vector through the 3x3 part: lane i is
        /// ((row[i][1]*y + row[i][0]*x) + row[i][2]*z), rows at stride 16.
        unsafe fn kernel(m: u32, x: f32, y: f32, z: f32) -> (f32, f32, f32) {
            unsafe {
                let r0 = add(add(mul(rdf(m + 0x10), y), mul(rdf(m), x)), mul(rdf(m + 0x20), z));
                let r1 = add(add(mul(rdf(m + 0x14), y), mul(rdf(m + 4), x)), mul(rdf(m + 0x24), z));
                let r2 = add(add(mul(rdf(m + 0x18), y), mul(rdf(m + 8), x)), mul(rdf(m + 0x28), z));
                (r0, r1, r2)
            }
        }

        let flags = ((this.wrapping_add(FLAGS)) as *const u8).read();
        if flags & 4 == 0 {
            return 0;
        }
        let driver = rd32(this.wrapping_add(DRIVER));
        if flags & 2 == 0 || driver == 0 {
            for i in 0..4u32 {
                wr32(this.wrapping_add(DST).wrapping_add(i * 4), rd32(this.wrapping_add(SRC).wrapping_add(i * 4)));
            }
            return 0;
        }
        let selector = rd32(this.wrapping_add(SELECTOR));
        if selector != NO_SELECTOR && rd32(driver.wrapping_add(DRV_KIND)) & KIND_MASK == KIND_WANT {
            let ready: u32 = lf_checker_rt::callee_thiscall!(READY, u32, driver);
            let x = rdf(this.wrapping_add(SRC));
            let y = rdf(this.wrapping_add(SRC).wrapping_add(4));
            let z = rdf(this.wrapping_add(SRC).wrapping_add(8));
            if ready != 0 {
                let mut arr = [
                    f32::from_bits(lf_checker_rt::global::<u32>(OFF_G0).read()),
                    f32::from_bits(lf_checker_rt::global::<u32>(OFF_G1).read()),
                    f32::from_bits(lf_checker_rt::global::<u32>(OFF_G2).read()),
                    0.0f32,
                ];
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    MAPPING,
                    u32,
                    driver,
                    arr.as_mut_ptr() as u32,
                    selector,
                );
                let m: u32 = lf_checker_rt::callee_thiscall!(MATRIX, u32, driver);
                let (r0, r1, r2) = kernel(m, x, y, z);
                wrf(this.wrapping_add(DST), add(arr[0], r0));
                wrf(this.wrapping_add(DST).wrapping_add(4), add(arr[1], r1));
                wrf(this.wrapping_add(DST).wrapping_add(8), add(arr[2], r2));
                wrf(this.wrapping_add(DST).wrapping_add(12), arr[3]);
            } else {
                let m: u32 = lf_checker_rt::callee_thiscall!(MATRIX, u32, driver);
                let (r0, r1, r2) = kernel(m, x, y, z);
                wrf(this.wrapping_add(DST), add(r0, rdf(m + 0x30)));
                wrf(this.wrapping_add(DST).wrapping_add(4), add(r1, rdf(m + 0x34)));
                wrf(this.wrapping_add(DST).wrapping_add(8), add(r2, rdf(m + 0x38)));
                wr32(this.wrapping_add(DST).wrapping_add(12), 0);
            }
            return 0;
        }
        if rd32(driver.wrapping_add(DRV_MATRIX)) == 0 {
            let _: u32 = lf_checker_rt::callee_thiscall!(SETUP0, u32, driver);
            let p = rd32(driver.wrapping_add(DRV_MATRIX));
            let _: u32 = lf_checker_rt::callee_thiscall!(SETUP1, u32, driver.wrapping_add(0x10), p);
        }
        let m = rd32(driver.wrapping_add(DRV_MATRIX));
        let x = rdf(this.wrapping_add(SRC));
        let y = rdf(this.wrapping_add(SRC).wrapping_add(4));
        let z = rdf(this.wrapping_add(SRC).wrapping_add(8));
        let (r0, r1, r2) = kernel(m, x, y, z);
        wrf(this.wrapping_add(DST), add(r0, rdf(m + 0x30)));
        wrf(this.wrapping_add(DST).wrapping_add(4), add(r1, rdf(m + 0x34)));
        wrf(this.wrapping_add(DST).wrapping_add(8), add(r2, rdf(m + 0x38)));
        wr32(this.wrapping_add(DST).wrapping_add(12), 0);
        0
    }
});
