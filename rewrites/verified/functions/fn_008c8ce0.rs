// original: 0x008C8CE0 stream_anchor_nearest
/// Find the active anchor nearest to the streaming focus point.
///
/// Takes the focus point from the focus callee's anchor (reached through
/// the word at +0x20 of its object, point at +0x30/+0x34 of the anchor)
/// or the origin when it reports none, then scans the anchor
/// table for the active entry (flag byte at +0x38 set) with the smallest
/// squared distance that still beats the running best, seeded from the
/// threshold global. The winning index lands in the result global (left
/// at -1 when nothing wins) and the entry count is returned. All
/// arithmetic is single-precision SSE in the original's operand order.
/// Original: cdecl, no stack words.
lf_checker_rt::export!(cdecl, rw_008c8ce0() -> u32 {
    unsafe {
        use core::arch::x86::{
            _mm_cvtss_f32, _mm_set_ss, _mm_sub_ss, _mm_mul_ss, _mm_add_ss,
        };
        const FOCUS_CALLEE: u32 = 1;
        const ENTRY_STRIDE: u32 = 0x40;
        const ACTIVE_OFF: u32 = 0x38;
        #[inline(always)]
        unsafe fn sub(a: f32, b: f32) -> f32 {
            _mm_cvtss_f32(_mm_sub_ss(
                _mm_set_ss(core::hint::black_box(a)),
                _mm_set_ss(core::hint::black_box(b))))
        }
        #[inline(always)]
        unsafe fn mul(a: f32, b: f32) -> f32 {
            _mm_cvtss_f32(_mm_mul_ss(
                _mm_set_ss(core::hint::black_box(a)),
                _mm_set_ss(core::hint::black_box(b))))
        }
        #[inline(always)]
        unsafe fn add(a: f32, b: f32) -> f32 {
            _mm_cvtss_f32(_mm_add_ss(
                _mm_set_ss(core::hint::black_box(a)),
                _mm_set_ss(core::hint::black_box(b))))
        }
        let obj: u32 = lf_checker_rt::callee_cdecl!(FOCUS_CALLEE, u32,);
        let (qx, qy) = if obj == 0 {
            (0.0f32, 0.0f32)
        } else {
            let anchor =
                ((obj + 0x20) as *const u32).read_unaligned();
            (((anchor + 0x30) as *const f32).read_unaligned(),
             ((anchor + 0x34) as *const f32).read_unaligned())
        };
        let count = *lf_checker_rt::global::<i32>(0x12DD738);
        let mut best =
            *lf_checker_rt::global::<f32>(0x00FE8D18);
        *lf_checker_rt::global::<u32>(0x1032110) = 0xFFFF_FFFF;
        if count > 0 {
            let table = lf_checker_rt::relocated(0x12DDB00);
            let mut i: i32 = 0;
            while i < count {
                let row = table.wrapping_add(
                    (i as u32).wrapping_mul(ENTRY_STRIDE));
                if ((row + ACTIVE_OFF) as *const u8).read() != 0 {
                    let ex = (row as *const f32).read_unaligned();
                    let ey = ((row + 4) as *const f32).read_unaligned();
                    let dy = sub(qy, ey);
                    let dx = sub(qx, ex);
                    let d = add(mul(dy, dy), mul(dx, dx));
                    // comiss best, d + jbe keeps a strictly decreasing
                    // minimum: ties and unordered pairs keep the old best.
                    if d < best {
                        best = d;
                        *lf_checker_rt::global::<u32>(0x1032110) = i as u32;
                    }
                }
                i += 1;
            }
        }
        if count > 0 {
            count as u32
        } else {
            0
        }
    }
});
