// original: 0x009D25B0 bbox_merge_minmax (proposed)
//
/// Merges another bounding box into this one, componentwise.
///
/// The first three floats (at `+0x00..+0x08`) keep the componentwise minimum
/// and the last three (at `+0x10..+0x18`) the componentwise maximum of this
/// box and `other`, widening this box to enclose both. Each lane is a pure
/// selection driven by an ordered floating comparison (`a > b` is false when
/// either side is NaN, matching the original's `comiss` + `ja`), so the
/// result is bit-exact including NaNs and signed zeros. Returns nothing
/// meaningful (`eax` is untouched passthrough). Thiscall, one pointer argument.
lf_checker_rt::export!(thiscall, rw_009D25B0(this: u32, other: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rf(p: u32) -> f32 {
            unsafe { f32::from_bits((p as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        unsafe fn wf(p: u32, v: f32) {
            unsafe { (p as *mut u32).write_unaligned(v.to_bits()) }
        }
        // Minimum lanes.
        for off in [0x00u32, 0x04, 0x08] {
            let t = rf(this.wrapping_add(off));
            let o = rf(other.wrapping_add(off));
            wf(this.wrapping_add(off), if o > t { t } else { o });
        }
        // Maximum lanes.
        for off in [0x10u32, 0x14, 0x18] {
            let t = rf(this.wrapping_add(off));
            let o = rf(other.wrapping_add(off));
            wf(this.wrapping_add(off), if t > o { t } else { o });
        }
        0
    }
});
