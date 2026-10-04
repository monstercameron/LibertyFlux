// original: 0x00b38620 pool_group_flag_distant_idle (proposed)

/// Scan one of sixteen groups of the ped pool and flag members that are far
/// from a point and idle.
///
/// `target` points to three floats (x, y, z). The pool header is read from a
/// global: item array at `+0x00`, flag bytes at `+0x04`, item count at `+0x08`
/// and item stride at `+0x0c`. A second global masked to four bits selects
/// which sixteenth of the pool to scan: with group `g` and count `n` the loop
/// covers `g*n/16 .. (g+1)*n/16` with signed division that truncates toward
/// zero, so a non-positive count scans nothing.
///
/// For each index in the range: members whose flag byte has bit 0x80 set are
/// skipped, as are null slots. A live member is polled twice through virtual
/// slot `+0xd4` (thiscall, no stack arguments); a zero first answer skips it,
/// otherwise a 16-bit kind field at `+0x2e` (sign-extended) and the second
/// answer go to a direct thiscall check. Members with a non-null link at
/// `+0x6c` are skipped (at once when the byte at link `+0x0e` is set,
/// otherwise after the distance test). The squared distance from the member's
/// position (three floats at `+0x30` of the object at `+0x20`) to the target
/// must be ordered-greater than the 225.0 limit; then a final direct thiscall
/// check with argument 1 must answer zero before the member is reported
/// through a cdecl call with `(member, 1)`.
///
/// Original: 0x00b38620 (cdecl, one stack word, no return value).
lf_checker_rt::export!(cdecl, rw_00b38620(target: u32) -> u32 {
    unsafe {
        const POOL_GLOBAL: u32 = 0x018b6f10;
        const GROUP_GLOBAL: u32 = 0x01173604;
        const DIST_LIMIT: u32 = 0x00fe8c00;
        const POOL_ITEMS: u32 = 0x00;
        const POOL_FLAGS: u32 = 0x04;
        const POOL_COUNT: u32 = 0x08;
        const POOL_STRIDE: u32 = 0x0c;
        const SKIP_BIT: u8 = 0x80;
        const VT_SLOT_POLL: u32 = 0xd4;
        const MEMBER_POS: u32 = 0x20;
        const MEMBER_KIND: u32 = 0x2e;
        const MEMBER_LINK: u32 = 0x6c;
        const LINK_IDLE: u32 = 0x0e;
        const POS_X: u32 = 0x30;
        const CALLEE_POLL: u32 = 1;
        const CALLEE_GATE: u32 = 2;
        const CALLEE_IDLE: u32 = 3;
        const CALLEE_FLAG: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16s(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn poll(member: u32) -> u32 {
            unsafe {
                let vt = rd32(member);
                let slot = rd32(vt.wrapping_add(VT_SLOT_POLL));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(member)
            }
        }

        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let group = rd32(lf_checker_rt::relocated(GROUP_GLOBAL)) & 0xf;
        let count = rd32(pool.wrapping_add(POOL_COUNT)) as i32;
        // Signed truncating division by 16, as the original's
        // cdq/and/add/sar sequence computes it.
        let lo = (group as i32).wrapping_mul(count) / 16;
        let hi = ((group as i32).wrapping_add(1)).wrapping_mul(count) / 16;
        let limit = f32::from_bits(rd32(lf_checker_rt::relocated(DIST_LIMIT)));
        let mut i = lo;
        while i < hi {
            let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
            let flags = rd32(pool.wrapping_add(POOL_FLAGS));
            if rd8(flags.wrapping_add(i as u32)) & SKIP_BIT != 0 {
                i = i.wrapping_add(1);
                continue;
            }
            let stride = rd32(pool.wrapping_add(POOL_STRIDE));
            let base = rd32(pool.wrapping_add(POOL_ITEMS));
            let member = base.wrapping_add(stride.wrapping_mul(i as u32));
            if member == 0 {
                i = i.wrapping_add(1);
                continue;
            }
            if poll(member) == 0 {
                i = i.wrapping_add(1);
                continue;
            }
            let kind = rd16s(member.wrapping_add(MEMBER_KIND));
            let gate_this = poll(member);
            let keep = lf_checker_rt::callee_thiscall!(CALLEE_GATE, u32, gate_this, kind as u32);
            if (keep & 0xff) == 0 {
                i = i.wrapping_add(1);
                continue;
            }
            let link = rd32(member.wrapping_add(MEMBER_LINK));
            if link != 0 && rd8(link.wrapping_add(LINK_IDLE)) != 0 {
                i = i.wrapping_add(1);
                continue;
            }
            let pos = rd32(member.wrapping_add(MEMBER_POS));
            let dx = sub(
                f32::from_bits(rd32(pos.wrapping_add(POS_X))),
                f32::from_bits(rd32(target)),
            );
            let dy = sub(
                f32::from_bits(rd32(pos.wrapping_add(POS_X + 4))),
                f32::from_bits(rd32(target.wrapping_add(4))),
            );
            let dz = sub(
                f32::from_bits(rd32(pos.wrapping_add(POS_X + 8))),
                f32::from_bits(rd32(target.wrapping_add(8))),
            );
            let dist2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
            // comiss + jbe: continue unless ordered-greater.
            if !(dist2 > limit) {
                i = i.wrapping_add(1);
                continue;
            }
            let busy = lf_checker_rt::callee_thiscall!(CALLEE_IDLE, u32, member, 1);
            if (busy & 0xff) != 0 {
                i = i.wrapping_add(1);
                continue;
            }
            if rd32(member.wrapping_add(MEMBER_LINK)) != 0 {
                i = i.wrapping_add(1);
                continue;
            }
            lf_checker_rt::callee_cdecl!(CALLEE_FLAG, u32, member, 1);
            i = i.wrapping_add(1);
        }
        0
    }
});
