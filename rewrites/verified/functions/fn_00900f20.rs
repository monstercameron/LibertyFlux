// original: 0x00900f20 ui_slot_range_update (proposed)

/// Validate a UI slot against the pad state and a range window, then scale
/// the caller's factor and record the slot index.
///
/// `idx` indexes the slot table at `0x118F6F8`; each slot holds an active
/// byte at `+8`, a mode word at `+0x4c`, and two float pairs. `scale_ptr`
/// points to the caller's factor float. Returns 1 on success, 0 on any
/// rejection.
///
/// Behaviour: callee 1 (with 0) must answer nonzero, and `idx` must differ
/// from the global at `0x1034494`, else return 0. Read the slot's active
/// byte and mode word (from the global-selected slot when inactive): a mode
/// of 3 accepts at once, a mode of 6 accepts at once, otherwise callee 2 is
/// asked with `idx` twice and must answer 4 then anything, or anything then
/// 2 (both answers share one scripted value per trial, so 4 skips the second
/// call and 2 passes both). A float triple is then copied out of the slot
/// (offsets `+0x30/34/38` when active, `+0x20/24` plus zero) into dead stack
/// slots: unobservable, but the reads are kept for fault parity. Three work
/// cells start at the global base 0.5 (`0xFE8830`); callee 3 (with 1) must
/// return a block whose byte at `+0x328d` is nonzero for the compute path,
/// else the cells keep base/zero and control joins at the second callee-5
/// call. The compute path converts the global ints at `0x18B7A80/8C` to
/// float, scales them by the globals at `0x17ACCE8/F0`, calls callee 4 (the
/// range curve) with the first two cells and the relocated `0x1190E70`
/// (which may refill the second cell), then calls callee 5 with the third
/// cell, the second cell, `0x1190E70` and 0 (which refills both cells) and
/// copies the second cell over the first. The join calls callee 5 again with
/// the triple head and the third cell. Four window checks against the
/// half-width `w` from `0xFE871C` must then all pass: `c16 > c32 - w`,
/// `c32 + w > c16`, `c28 > c36 - w`, `c36 + w > c28`, where `c28` is the base
/// cell (always 0.5) and `c36` its copy on the compute path but stack the
/// original never wrote on the skip path (defined as 0 by the contract's
/// stack fill; NaN anywhere fails the check, matching `jbe`). On success
/// the factor at
/// `scale_ptr` is multiplied by the global at `0xFE891C`, `idx` is stored to
/// the global at `0x10344A8`, and 1 is returned.
///
/// Original: 0x00900f20 (cdecl, two stack words; returns al).
lf_checker_rt::export!(cdecl, rw_00900f20(idx: u32, scale_ptr: u32) -> u32 {
    unsafe {
        const SLOTS: u32 = 0x0118_f6f8;
        const CUR_SEL: u32 = 0x0103_4494;
        const LAST_SEL: u32 = 0x0103_44a8;
        const ACTIVE: u32 = 0x08;
        const MODE: u32 = 0x4c;
        const BASE: u32 = 0x00fe_8830;
        const WINDOW: u32 = 0x00fe_871c;
        const SCALE: u32 = 0x00fe_891c;
        const SRC0: u32 = 0x018b_7a80;
        const SRC1: u32 = 0x018b_7a8c;
        const MUL0: u32 = 0x017a_cce8;
        const MUL1: u32 = 0x017a_ccf0;
        const PAD_FLAG: u32 = 0x328d;
        const AUX_TABLE: u32 = 0x0119_0e70;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
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
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn slot(i: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(SLOTS).wrapping_add(i.wrapping_mul(4))) }
        }

        let ok = lf_checker_rt::callee_cdecl!(1, u32, 0) as u8;
        if ok == 0 {
            return 0;
        }
        let cur = rd32(lf_checker_rt::relocated(CUR_SEL));
        if idx == cur {
            return 0;
        }
        let esi = slot(idx);
        let al0 = rd8(esi.wrapping_add(ACTIVE));
        let mode_of = |p: u32| rd32(p.wrapping_add(MODE));
        let edx = if al0 != 0 { mode_of(esi) } else { mode_of(slot(cur)) };
        if edx != 3 {
            let eax0 = if al0 != 0 { mode_of(esi) } else { mode_of(slot(cur)) };
            if eax0 != 6 {
                let r1 = lf_checker_rt::callee_cdecl!(2, u32, idx);
                if r1 != 4 {
                    let r2 = lf_checker_rt::callee_cdecl!(2, u32, idx);
                    if r2 != 2 {
                        return 0;
                    }
                }
            }
        }
        // Slot triple: the first word feeds the join call below (live),
        // the rest are dead stores; keep their reads for fault parity.
        #[repr(C)]
        struct Pair {
            c16: f32,
            c20: f32,
        }
        let base = rdf(lf_checker_rt::relocated(BASE));
        // c16/c20 share one two-word snap, so they share a repr(C) pair; the
        // other cells are single-word snap/write cells. c28 never changes
        // after init; c36 is base on the compute path, fill 0 on the skip.
        let c28 = base;
        let mut pr = Pair { c16: base, c20: 0.0 };
        let mut c24 = base;
        let mut c32: f32 = 0.0;
        let mut c36: f32 = 0.0;
        let mut t48: f32 = 0.0;
        if al0 != 0 {
            t48 = f32::from_bits(rd32(esi.wrapping_add(0x30)));
            core::hint::black_box(rd32(esi.wrapping_add(0x34)));
            core::hint::black_box(rd32(esi.wrapping_add(0x38)));
        } else {
            t48 = f32::from_bits(rd32(esi.wrapping_add(0x20)));
            core::hint::black_box(rd32(esi.wrapping_add(0x24)));
        }
        let pad = lf_checker_rt::callee_cdecl!(3, u32, 1);
        if rd8(pad.wrapping_add(PAD_FLAG)) != 0 {
            let i1 = rd32(lf_checker_rt::relocated(SRC0)) as i32 as f32;
            let m1 = rdf(lf_checker_rt::relocated(MUL0));
            let i2 = rd32(lf_checker_rt::relocated(SRC1)) as i32 as f32;
            let m2 = rdf(lf_checker_rt::relocated(MUL1));
            pr.c16 = mul(i1, m1);
            pr.c20 = mul(i2, m2);
            lf_checker_rt::callee_cdecl!(
                4,
                u32,
                core::ptr::addr_of_mut!(pr.c16) as u32,
                core::ptr::addr_of_mut!(c24) as u32,
                lf_checker_rt::relocated(AUX_TABLE)
            );
            c32 = c24;
            c36 = c28;
            lf_checker_rt::callee_cdecl!(
                5,
                u32,
                core::ptr::addr_of_mut!(c32) as u32,
                core::ptr::addr_of_mut!(c24) as u32,
                lf_checker_rt::relocated(AUX_TABLE),
                0
            );
            pr.c16 = c24;
        }
        lf_checker_rt::callee_cdecl!(
            5,
            u32,
            core::ptr::addr_of_mut!(t48) as u32,
            core::ptr::addr_of_mut!(c32) as u32,
            lf_checker_rt::relocated(AUX_TABLE),
            0
        );
        let w = rdf(lf_checker_rt::relocated(WINDOW));
        if !(pr.c16 > sub(c32, w)) {
            return 0;
        }
        if !(add(c32, w) > pr.c16) {
            return 0;
        }
        if !(c28 > sub(c36, w)) {
            return 0;
        }
        if !(add(c36, w) > c28) {
            return 0;
        }
        let x = rdf(scale_ptr);
        let s = rdf(lf_checker_rt::relocated(SCALE));
        wrf(scale_ptr, mul(x, s));
        wr32(lf_checker_rt::relocated(LAST_SEL), idx);
        1
    }
});
