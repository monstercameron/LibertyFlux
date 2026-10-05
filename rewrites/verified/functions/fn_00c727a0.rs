// original: 0x00c727a0 ped_task_target_update (proposed)

/// Refresh a ped's task target from a spatial query and a fallback finder.
///
/// `point` points to two floats (x, y). When the scan flag byte is set and
/// the spatial index global is nonzero, the routine queries the index
/// (intercepted thiscall: the index, a four-float box, and an out-slot) for
/// a 500-unit box around the point, laid out as x-500, y+500, x+500, y-500,
/// walks the returned node chain to its end through each node's `+4` link
/// (the nodes themselves are unused), and releases the list (intercepted
/// thiscall of no arguments). It then checks the current target global: when
/// nonzero, the target validator (intercepted thiscall of the point) runs,
/// and a passing target together with the finder flag set returns early.
/// Otherwise, depending on the clear flag byte, a stale nonzero target is
/// dropped through the target setter (intercepted cdecl callee of two
/// arguments) and the global is cleared. Finally, when the finder flag is
/// set, the box finder (intercepted cdecl callee of the point) runs; a
/// nonzero result is installed through the target setter and stored to the
/// target global.
///
/// Original: 0x00C727A0 (cdecl, one stack argument; no defined return).
lf_checker_rt::export!(cdecl, rw_00C727A0(point: u32) -> u32 {
    unsafe {
        const SCAN_FLAG: u32 = 0x15B0E59;
        const CLEAR_FLAG: u32 = 0x15B0E5A;
        const FINDER_FLAG: u32 = 0x103F6D7;
        const INDEX_GLOBAL: u32 = 0x16DD680;
        const TARGET_GLOBAL: u32 = 0x16DD67C;
        const RADIUS: u32 = 0xFE8C2C; // 500.0
        const QUERY: u32 = 1;
        const RELEASE: u32 = 2;
        const VALIDATE: u32 = 3;
        const SET_TARGET: u32 = 4;
        const FIND_BOX: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { (lf_checker_rt::global::<u8>(va) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { (lf_checker_rt::global::<u32>(va) as *const u32).read() }
        }
        #[inline(always)]
        unsafe fn set_g32(va: u32, v: u32) {
            unsafe { (lf_checker_rt::global::<u32>(va) as *mut u32).write(v) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        if g8(SCAN_FLAG) != 0 {
            let index = g32(INDEX_GLOBAL);
            if index != 0 {
                let x = rdf(point);
                let y = rdf(point + 4);
                let r = f32::from_bits(g32(RADIUS));
                let x0 = sub(x, r);
                let x1 = add(x, r);
                let y0 = sub(y, r);
                let y1 = add(y, r);
                let mins = [x0.to_bits(), y1.to_bits(), x1.to_bits(), y0.to_bits()];
                let mut head: u32 = 0;
                let head_ptr = (&mut head as *mut u32) as u32;
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    QUERY,
                    u32,
                    index,
                    mins.as_ptr() as u32,
                    head_ptr
                );
                let mut node = head;
                while node != 0 {
                    node = rd32(node + 4);
                }
                let _: u32 = lf_checker_rt::callee_thiscall!(RELEASE, u32, head_ptr);
            }
        }
        let mut target = g32(TARGET_GLOBAL);
        if target != 0 {
            let ok: u32 = lf_checker_rt::callee_thiscall!(VALIDATE, u32, target, point);
            if (ok as u8) != 0 && g8(FINDER_FLAG) != 0 {
                return 0;
            }
            target = g32(TARGET_GLOBAL);
        }
        if g8(CLEAR_FLAG) == 0 {
            if target != 0 {
                let _: u32 = lf_checker_rt::callee_cdecl!(SET_TARGET, u32, target, 0);
                set_g32(TARGET_GLOBAL, 0);
            }
        } else if target != 0 {
            return 0;
        }
        if g8(FINDER_FLAG) == 0 {
            return 0;
        }
        let found: u32 = lf_checker_rt::callee_cdecl!(FIND_BOX, u32, point);
        if found == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(SET_TARGET, u32, found, 1);
        set_g32(TARGET_GLOBAL, found);
        0
    }
});
