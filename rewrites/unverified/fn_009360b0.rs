// original: 0x009360B0 net_dest_reach (proposed)

/// Decide whether a destination counts as reached, publishing an arrival.
///
/// `obj` is a route block: dword at `+0xd60` is a signed waypoint count,
/// dword at `+0xd64` a kind (4 and 5 never count), dword at `+0xd70` an
/// exactness flag. A mode global selects an entity through the first callee;
/// when the count is not positive the entity index must be positive too or
/// the answer is 0. The target point is the last 16-byte waypoint in the
/// block when the count is positive, otherwise the entity position supplied
/// by the third callee; the reference point is three words the second callee
/// writes. The squared planar distance must be below the squared slop from
/// a global, and the absolute height gap below one of two read-only
/// altitude limits chosen by a flag byte. A set exactness flag additionally
/// requires a strict-mode byte to be clear, and skips arrival publishing;
/// otherwise, when reached, an arrival probe runs and, unless this arrival
/// was already published, a 14-word descriptor block shared by the last two
/// callees is sent and the published flag cleared. A reached route with a
/// positive count has its kind reset to 0. Returns 1 when reached, 0 when
/// not; like the original's byte-wide result writes, only the low byte is
/// set and the rest of EAX is whatever the last value-producing step left.
/// thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_009360B0(obj: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0xd60;
        const KIND_OFF: u32 = 0xd64;
        const EXACT_OFF: u32 = 0xd70;
        const WP_STRIDE: u32 = 16;
        const G_MODE: u32 = 0x1160C4C;
        const G_ALT: u32 = 0x11A2EA3;
        const G_STRICT: u32 = 0x11A2EA8;
        const G_FIRST: u32 = 0x1036EEC;
        const G_SLOP: u32 = 0x1036ED8;
        const ALT0: u32 = 0xFE8B40;
        const ALT1: u32 = 0xFE8AD8;
        const SIGN: u32 = 0x8000_0000;
        const OBJ_A: u32 = 0x116BFF0;
        const OBJ_B: u32 = 0x1033130;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        fn bits(f: f32) -> u32 {
            f.to_bits()
        }
        #[inline(always)]
        fn f(b: u32) -> f32 {
            f32::from_bits(b)
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
        fn absf(a: f32) -> f32 {
            f32::from_bits(a.to_bits() & !SIGN)
        }

        let g = lf_checker_rt::relocated;
        let edi = lf_checker_rt::callee_cdecl!(1, u32, rd32(g(G_MODE)));
        let count = rd32(obj.wrapping_add(COUNT_OFF)) as i32;
        if count <= 0 && (edi as i32) <= 0 {
            return edi & !0xFF;
        }
        let kind = rd32(obj.wrapping_add(KIND_OFF));
        if kind == 4 || kind == 5 {
            return kind & !0xFF;
        }
        let mut refpt = [0u32; 3];
        lf_checker_rt::callee_cdecl!(2, u32, refpt.as_mut_ptr() as u32);
        let (x, y, z) = if count > 0 {
            let base = obj.wrapping_add(
                (count as u32).wrapping_mul(2).wrapping_mul(8),
            );
            (
                f(rd32(base.wrapping_sub(WP_STRIDE))),
                f(rd32(base.wrapping_sub(WP_STRIDE - 4))),
                f(rd32(base.wrapping_sub(WP_STRIDE - 8))),
            )
        } else {
            let mut ebuf = [0u32; 3];
            let p = lf_checker_rt::callee_cdecl!(3, u32, ebuf.as_mut_ptr() as u32, edi);
            (f(rd32(p)), f(rd32(p.wrapping_add(4))), f(rd32(p.wrapping_add(8))))
        };
        let alt_va = if rd8(g(G_ALT)) == 0 { ALT0 } else { ALT1 };
        let dy = sub(y, f(refpt[1]));
        let dx = sub(x, f(refpt[0]));
        let d2 = add(mul(dy, dy), mul(dx, dx));
        let alt = f(rd32(g(alt_va)));
        let exact = rd32(obj.wrapping_add(EXACT_OFF));
        if exact != 0 && rd8(g(G_STRICT)) != 0 {
            return exact & !0xFF;
        }
        let slop = f(rd32(g(G_SLOP)));
        if !(mul(slop, slop) > d2) {
            return exact & !0xFF;
        }
        let dz = absf(sub(z, f(refpt[2])));
        if !(alt > dz) {
            return exact & !0xFF;
        }
        let prev: u32;
        if exact == 0 {
            let a4 = lf_checker_rt::callee_cdecl!(4, u32,);
            prev = a4;
            if rd8(g(G_FIRST)) != 0 {
                let a5 = lf_checker_rt::callee_thiscall!(
                    5, u32, g(OBJ_A), g(0xE87900)
                );
                let a6 = lf_checker_rt::callee_thiscall!(
                    6, u32, g(OBJ_B),
                    a5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0xFFFF_FFFF
                );
                prev = a6;
                wr8(g(G_FIRST), 0);
            }
        } else {
            prev = exact;
        }
        if count > 0 {
            wr32(obj.wrapping_add(KIND_OFF), 0);
        }
        (prev & !0xFF) | 1
    }
});
