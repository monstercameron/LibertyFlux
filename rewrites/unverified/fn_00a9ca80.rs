// original: 0x00a9ca80 stream_flood_mark (proposed)

/// Mark one graph node and recurse into unmarked non-negative neighbours.
///
/// `base` points to an array of 28-byte nodes; node `v` carries its mark
/// at `base + v * 28 + 0x18` and three neighbour indexes at
/// `base + index * 28 + 0x0c`. The current node `index` is marked with
/// `fill`. Each neighbour is then visited (through the intercepted
/// recursive call, so the checker observes one call per visit instead of
/// real recursion) when it is non-negative and its own mark is still `-1`.
/// `opaque` is never read, only forwarded to the recursive calls. The entry
/// object pointer is forwarded unchanged as well. The return value is the
/// last neighbour word read, or the last recursive answer.
///
/// Original: 0x00a9ca80 (thiscall, four stack words; self-recursive, the
/// recursion site is intercepted).
lf_checker_rt::export!(thiscall, rw_00a9ca80(this: u32, base: u32, opaque: u32, index: u32, fill: u32) -> u32 {
    unsafe {
        const MARK_DELTA: u32 = 0x18;
        const LINKS_DELTA: u32 = 0x0c;
        const NODE_STRIDE: u32 = 28;
        const LINK_COUNT: u32 = 3;
        const UNMARKED: u32 = 0xffff_ffff;
        const RECURSE: u32 = 1;
        let stride = index.wrapping_mul(NODE_STRIDE);
        ((base + MARK_DELTA + stride) as *mut u32).write_unaligned(fill);
        let mut j = 0u32;
        let mut seen = 0u32;
        while j < LINK_COUNT {
            let v = ((base + LINKS_DELTA + stride + j.wrapping_mul(4)) as *const u32)
                .read_unaligned();
            seen = v;
            if (v as i32) >= 0 {
                let mark = ((base + MARK_DELTA + v.wrapping_mul(NODE_STRIDE)) as *const u32)
                    .read_unaligned();
                if mark == UNMARKED {
                    seen = lf_checker_rt::callee_thiscall!(
                        RECURSE, u32, this, base, opaque, v, fill
                    );
                }
            }
            j += 1;
        }
        seen
    }
});
