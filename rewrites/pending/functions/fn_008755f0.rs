// original: 0x008755f0 rage::crmtRequestAnimation::vf2
/// Build the evaluated output of an animation motion request.
///
/// Creates the output object through the allocator helper (stubbed),
/// takes a reference on this request's child, moves it into the output
/// slot `0x34` (releasing the displaced child through vtable slot 0 when
/// its count reaches zero), clamps the blend weight into output offset
/// `0x20`, then copies the blend factor, merges the two mode bytes into
/// the flag word at `0x1C`, copies the extra mode byte, and runs the
/// finish helper (stubbed) when the flag argument is non-zero. Returns
/// the output object.
///
/// The first of the two weight clamps in the original is dead (its store
/// is overwritten unconditionally) but is reproduced for fidelity; both
/// clamps use ordered floating comparisons so NaN propagates exactly as
/// the original's predicated moves do.
export!(thiscall, rw_008755f0(this: u32, tag: u32, finish: u32) -> u32 {
    unsafe {
        const CHILD: usize = 0x14 / 4;
        const WEIGHT: usize = 0x18 / 4;
        const FACTOR: usize = 0x1C / 4;
        const OUT_CHILD: usize = 0x34 / 4;
        const OUT_WEIGHT: usize = 0x20 / 4;
        const OUT_FACTOR: usize = 0x30 / 4;
        const OUT_FLAGS: usize = 0x1C / 4;
        const CHILD_LIMIT: usize = 0xC / 4;
        let base = this as *const u32;
        let out: u32 = callee_cdecl!(1, u32, tag);
        let child = base.add(CHILD).read();
        let weight = f32::from_bits(base.add(WEIGHT).read());
        let factor = base.add(FACTOR).read();
        if child != 0 {
            let count = (child as *mut u16).add(2);
            count.write(count.read().wrapping_add(1));
        }
        let dst = out as *mut u32;
        let old = dst.add(OUT_CHILD).read();
        if old != 0 {
            let count = (old as *mut u16).add(2);
            let left = count.read().wrapping_sub(1);
            count.write(left);
            if left == 0 {
                let vt = (old as *const u32).read();
                let target = (vt as *const u32).read();
                let release: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(target as usize);
                let _: u32 = release(old, 1);
            }
        }
        dst.add(OUT_CHILD).write(child);
        if child != 0 {
            let current = f32::from_bits(dst.add(OUT_WEIGHT).read());
            let limit = f32::from_bits((child as *const u32).add(CHILD_LIMIT).read());
            // Dead clamp of the previous weight; kept for fidelity.
            let mut first = limit;
            if !(0.0f32 > current) {
                if !(current > first) {
                    first = current;
                }
            } else {
                first = 0.0;
            }
            dst.add(OUT_WEIGHT).write(first.to_bits());
        }
        dst.add(OUT_WEIGHT).write(weight.to_bits());
        if child != 0 {
            let limit = f32::from_bits((child as *const u32).add(CHILD_LIMIT).read());
            let selected: f32;
            if 0.0f32 > weight {
                selected = 0.0;
            } else if !(weight > limit) {
                selected = weight;
            } else {
                selected = limit;
            }
            dst.add(OUT_WEIGHT).write(selected.to_bits());
        }
        dst.add(OUT_FACTOR).write(factor);
        let mode0 = (this as *const u8).add(0x20).read();
        let mode1 = (this as *const u8).add(0x21).read();
        dst.add(OUT_FLAGS).write(dst.add(OUT_FLAGS).read() & 0xFFFFFFFC);
        if mode0 != 0 {
            let bits: u32 = if mode1 != 0 { 3 } else { 1 };
            dst.add(OUT_FLAGS).write(dst.add(OUT_FLAGS).read() | bits);
        }
        (out as *mut u8).add(0x38).write((this as *const u8).add(0x22).read());
        if finish != 0 {
            let _: u32 = callee_stdcall!(3, u32, out, 1);
        }
        out
    }
});
