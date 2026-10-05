// original: 0x00a2baa0 ped_task_approach_check

/// Decide whether a ped may start an approach task on a target.
///
/// `a1` is the ped, `a2` the candidate target. The answer is almost always
/// settled without floating point: the ped must carry the driver bit (byte
/// `+0x26c`, bit 2), must not already be busy (the shared flag from callee 0
/// combined with the virtual check in slot `+0x128`), the target must name a
/// task kind other than 0 or 1 (word at `a2+0x2b0`), and the ped's vehicle
/// record (`a1+0xb30`) must exist, must not be in state 1 (word `+0x1300`),
/// must pass callee 2 and must not carry bit `0x40` at `+0xf18`.
///
/// Task lookup (callee 3, thiscall on the task list at `a1[0x224]+0x2e0`)
/// then gates two deeper tests. The main test runs when the ped's current
/// task (callee 5's answer) reports busy through callee 6 while task
/// `0x419` exists: the displacement between the two matrices (`a1[0x20]`,
/// `a2[0x20]`, positions at `+0x30`) is dotted with the ped matrix's forward
/// row, and a positive value with task kind 2 or 3 accepts. The fallback
/// test runs when the main test is skipped: with vehicle state (`+0x1304`)
/// equal to 4 and task kind 1 or 3, a second dot-product (the ped matrix's
/// right row) is compared against the constant from the game data at file
/// `0xfe8628`, and a larger value accepts. Otherwise the current task head
/// (callee 7 on `a1[0x224]`) must exist, must identify as `0x419` through
/// its virtual slot `+0x0c`, and must be in sub-state 3, 4 or 5 (word
/// `+0x44`); anything else rejects.
///
/// The second dot-product in the main test recomputes a value the original
/// then discards; it has no observable effect and is not repeated here.
/// Only the low byte of the result is significant.
///
/// The original reads its constant through an unrelocated absolute address;
/// the rewrite reads the relocated copy.
///
/// Original: 0x00a2baa0 (cdecl, two stack words; returns low byte).
lf_checker_rt::export!(cdecl, rw_00a2baa0(a1: u32, a2: u32) -> u32 {
    unsafe {
        const DRIVER_BIT: u8 = 4;
        const VT_BUSY_SLOT: u32 = 0x128;
        const TASK_KIND: u32 = 0x2b0;
        const VEH: u32 = 0xb30;
        const VEH_STATE: u32 = 0x1300;
        const VEH_AUX: u32 = 0x1304;
        const TASK_HEAD: u32 = 0x224;
        const TASK_LIST: u32 = 0x2e0;
        const TASK_PROBE: u32 = 0x2e2;
        const TASK_MAIN: u32 = 0x419;
        const MATRIX: u32 = 0x20;
        const VT_ID_SLOT: u32 = 0x0c;
        const K_FACING: u32 = 0xfe8628;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn gf(ph: u32) -> f32 {
            unsafe {
                f32::from_bits(
                    (lf_checker_rt::global::<u32>(ph) as *const u32).read(),
                )
            }
        }
        #[inline(always)]
        unsafe fn vcall0(obj: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(obj).wrapping_add(slot)) as usize);
                f(obj)
            }
        }

        if rd8(a1.wrapping_add(0x26c)) & DRIVER_BIT == 0 {
            return 1;
        }
        if lf_checker_rt::callee_cdecl!(0, u32,) as u8 != 0
            && vcall0(a1, VT_BUSY_SLOT) as u8 != 0
        {
            return 1;
        }
        let kind = rd32(a2.wrapping_add(TASK_KIND));
        if kind == 0 || kind == 1 {
            return 0;
        }
        let veh = rd32(a1.wrapping_add(VEH));
        if veh == 0 {
            return 0;
        }
        if rd32(veh.wrapping_add(VEH_STATE)) == 1 {
            return 1;
        }
        if lf_checker_rt::callee_thiscall!(2, u32, veh) as u8 == 0 {
            return 1;
        }
        if rd8(rd32(a1.wrapping_add(VEH)).wrapping_add(0xf18)) & 0x40 != 0 {
            return 1;
        }
        let tasks = rd32(a1.wrapping_add(TASK_HEAD)).wrapping_add(TASK_LIST);
        if lf_checker_rt::callee_thiscall!(3, u32, tasks, TASK_PROBE, 0) as u8 != 0
            && lf_checker_rt::callee_thiscall!(4, u32, tasks, TASK_PROBE, 5) >= 0xb
        {
            return 1;
        }
        let busy = lf_checker_rt::callee_stdcall!(5, u32, a1);
        let main_ready = lf_checker_rt::callee_thiscall!(6, u32, a1) as u8 != 0
            && lf_checker_rt::callee_thiscall!(
                3, u32,
                rd32(a1.wrapping_add(TASK_HEAD)).wrapping_add(TASK_LIST),
                TASK_MAIN,
                0
            ) as u8
                != 0;
        if main_ready {
            let m1 = rd32(a1.wrapping_add(MATRIX));
            let m2 = rd32(a2.wrapping_add(MATRIX));
            let dx = sub(rdf(m2.wrapping_add(0x30)), rdf(m1.wrapping_add(0x30)));
            let dy = sub(rdf(m2.wrapping_add(0x34)), rdf(m1.wrapping_add(0x34)));
            let dz = sub(rdf(m2.wrapping_add(0x38)), rdf(m1.wrapping_add(0x38)));
            let d = add(
                add(mul(rdf(m1.wrapping_add(4)), dy), mul(rdf(m1), dx)),
                mul(rdf(m1.wrapping_add(8)), dz),
            );
            if d > 0.0 && (busy == 3 || busy == 2) {
                return 1;
            }
            return 1;
        }
        if rd32(rd32(a1.wrapping_add(VEH)).wrapping_add(VEH_AUX)) == 4 && (busy == 1 || busy == 3) {
            let m1 = rd32(a1.wrapping_add(MATRIX));
            let m2 = rd32(a2.wrapping_add(MATRIX));
            let dx = sub(rdf(m2.wrapping_add(0x30)), rdf(m1.wrapping_add(0x30)));
            let dy = sub(rdf(m2.wrapping_add(0x34)), rdf(m1.wrapping_add(0x34)));
            let dz = sub(rdf(m2.wrapping_add(0x38)), rdf(m1.wrapping_add(0x38)));
            let t = add(
                add(
                    mul(rdf(m1.wrapping_add(0x14)), dy),
                    mul(rdf(m1.wrapping_add(0x10)), dx),
                ),
                mul(rdf(m1.wrapping_add(0x18)), dz),
            );
            if t > gf(K_FACING) {
                return 1;
            }
        }
        let head = lf_checker_rt::callee_thiscall!(7, u32, rd32(a1.wrapping_add(TASK_HEAD)));
        if head == 0 {
            return 0;
        }
        if vcall0(head, VT_ID_SLOT) != TASK_MAIN {
            return 0;
        }
        if rd32(head.wrapping_add(0x44)).wrapping_sub(3) <= 2 {
            1
        } else {
            0
        }
    }
});
