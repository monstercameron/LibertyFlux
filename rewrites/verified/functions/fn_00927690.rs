// original: 0x00927690 input_ui_dispatch_scaled (proposed)

/// Query two metric objects, scale the results by integer division, and emit
/// them through a host object bracketed by begin/end calls.
///
/// `obj` supplies the base scale (virtual slot `MEASURE`), `cells` supplies
/// two quotients (slots `MEASURE` and `EXTENT`) divided by `divisor`. When
/// `mode` is 0 or 1 the two scaled products are
/// `other = (value - (value / divisor) * divisor) * (cells_measure / divisor)`
/// and `accum = (extent / divisor) * (value / divisor)` in mode 0 but
/// `accum = extent * (value / divisor)` in mode 1, where the extent quotient
/// is skipped (all 32-bit wrapping); any other mode sends zeroes. The middle call takes
/// the scale, both products, their sums with the scale, a 3-word vector read
/// from a global 16-byte constant (words 0..2), and a 5-word style block of
/// two unit floats, opaque black, and zeroes. The return value is the end
/// call's answer.
///
/// The original's mode-0 and mode-1 paths differ only in the extent
/// quotient (present in mode 0, skipped in mode 1). The divisor is never
/// zero in the proof (a zero divisor raises #DE
/// in the original but aborts in the rewrite, and the checker requires the
/// same fault code), and the `INT_MIN / -1` pairing is excluded for the same
/// reason.
///
/// Original: 0x00927690 (cdecl, five stack words).
lf_checker_rt::export!(cdecl, rw_00927690(obj: u32, cells: u32, value: u32, divisor: u32, mode: u32) -> u32 {
    unsafe {
        const SLOT_MEASURE: u32 = 0x20;
        const SLOT_EXTENT: u32 = 0x24;
        const SLOT_BEGIN: u32 = 0x3c;
        const SLOT_END: u32 = 0x40;
        const CALLEE_EMIT: u32 = 4;
        const HOST_GLOBAL: u32 = 0x017f5630;
        const VEC_GLOBAL: u32 = 0x00fe8e30;
        const UNIT: u32 = 0x3f800000;
        const OPAQUE_BLACK: u32 = 0xff000000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn vcall0(target: u32, slot: u32) -> u32 {
            unsafe {
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(target) + slot) as usize);
                f(target)
            }
        }

        let scale = vcall0(obj, SLOT_MEASURE);
        let d = divisor as i32;
        let (accum, other) = if mode == 0 {
            let q_cells = (vcall0(cells, SLOT_MEASURE) as i32) / d;
            let q_value = (value as i32) / d;
            let rem_scaled = (value as i32)
                .wrapping_sub(q_value.wrapping_mul(d))
                .wrapping_mul(q_cells);
            let q_extent = (vcall0(cells, SLOT_EXTENT) as i32) / d;
            (q_extent.wrapping_mul(q_value) as u32, rem_scaled as u32)
        } else if mode == 1 {
            let q_cells = (vcall0(cells, SLOT_MEASURE) as i32) / d;
            let q_value = (value as i32) / d;
            let rem_scaled = (value as i32)
                .wrapping_sub(q_value.wrapping_mul(d))
                .wrapping_mul(q_cells);
            let extent = vcall0(cells, SLOT_EXTENT) as i32;
            (extent.wrapping_mul(q_value) as u32, rem_scaled as u32)
        } else {
            (0, 0)
        };

        let host = rd32(lf_checker_rt::global::<u32>(HOST_GLOBAL) as u32);
        let begin: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(host) + SLOT_BEGIN) as usize);
        begin(host, 0, cells, 0, 0, 1, 0xffff_ffff);

        // Parameter block matching the original's frame layout: the vector
        // at +0x00 (snapshot 3) and the style block at +0x14 (snapshot 5).
        // Index 4 is uninitialized stack in the original and is never
        // snapshotted; it reads zero here.
        let vec = lf_checker_rt::global::<u32>(VEC_GLOBAL);
        let params = [
            vec.add(0).read_unaligned(),
            vec.add(1).read_unaligned(),
            vec.add(2).read_unaligned(),
            vec.add(3).read_unaligned(),
            0u32,
            UNIT,
            UNIT,
            OPAQUE_BLACK,
            0u32,
            0u32,
            0u32,
        ];
        lf_checker_rt::callee_cdecl!(
            CALLEE_EMIT,
            u32,
            cells,
            obj,
            other,
            accum,
            other.wrapping_add(scale),
            accum.wrapping_add(scale),
            0,
            0,
            0,
            scale,
            scale,
            params.as_ptr() as u32,
            0
        );

        let host2 = rd32(lf_checker_rt::global::<u32>(HOST_GLOBAL) as u32);
        let end: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(rd32(rd32(host2) + SLOT_END) as usize);
        end(host2, 0, params.as_ptr().add(5) as u32, 0xffff_ffff)
    }
});
