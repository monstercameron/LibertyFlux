// original: 0x00c8bdd0 audio_zone_register (proposed)
///
/// Registers the zone spanned by the two corner vectors `a` and `b`
/// (three floats each) unless the zone table is full. The table holds 16
/// rows of 0x30 bytes at file global 0x16fc690 with its count at file
/// global 0x16fc678; a count of 16 registers nothing. Otherwise the
/// per-component minimum and maximum are selected with the original's
/// compare semantics (minimum picks B unless B is ordered-above A, so an
/// unordered NaN pair picks B; maximum picks A only when A is
/// ordered-above B, else B) and handed to callee 1 as (row, minptr,
/// maxptr), where row is the count-th table row; then the three tag bytes
/// `t0..t2` are stored at row+0x20..0x22 and the count is raised by one.
/// Cdecl, five stack arguments (a, b, t0, t1, t2); no return value.

lf_checker_rt::export!(cdecl, rw_00c8bdd0(a: u32, b: u32, t0: u32, t1: u32, t2: u32) -> u32 {
    unsafe {
    #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
    #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
    #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits(rd32(a)) }
    }
        const COUNT: u32 = 0x16fc678;
        const TABLE: u32 = 0x16fc690;
        const CAP: u32 = 16;
        const PITCH: u32 = 0x30;
        const TAG_OFF: u32 = 0x20;
        let count = rd32(lf_checker_rt::relocated(COUNT));
        if count == CAP {
            return 0;
        }
        let mut mins = [0f32; 3];
        let mut maxs = [0f32; 3];
        let mut i: u32 = 0;
        while i < 3 {
            let va = rdf(a.wrapping_add(i.wrapping_mul(4)));
            let vb = rdf(b.wrapping_add(i.wrapping_mul(4)));
            let va = core::hint::black_box(va);
            let vb = core::hint::black_box(vb);
            mins[i as usize] = if !(vb > va) { vb } else { va };
            maxs[i as usize] = if va > vb { va } else { vb };
            i += 1;
        }
        let row = lf_checker_rt::relocated(TABLE).wrapping_add(count.wrapping_mul(PITCH));
        lf_checker_rt::callee_thiscall!(1, u32, row, mins.as_ptr() as u32, maxs.as_ptr() as u32);
        // (reloaded after the call, like the original)
        let count2 = rd32(lf_checker_rt::relocated(COUNT));
        let row2 = lf_checker_rt::relocated(TABLE).wrapping_add(count2.wrapping_mul(PITCH));
        ((row2.wrapping_add(TAG_OFF)) as *mut u8).write(t0 as u8);
        ((row2.wrapping_add(TAG_OFF + 1)) as *mut u8).write(t1 as u8);
        ((row2.wrapping_add(TAG_OFF + 2)) as *mut u8).write(t2 as u8);
        wr32(lf_checker_rt::relocated(COUNT), count2.wrapping_add(1));
        0
    }
});
