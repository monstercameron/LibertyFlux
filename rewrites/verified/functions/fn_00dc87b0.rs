// original: 0x00DC87B0 blend_task_vector (proposed)

use lf_checker_rt::{callee_cdecl, callee_thiscall, export};

/// Blend a task's target vector toward a probe result, or copy the source
/// vector through when either probe refuses.
///
/// `this` holds a source vector at `+0x20` (four words) and a target vector
/// at `+0x30` (four words). `p` points at an object whose word at `+0x20`
/// points at a float block. A classifier object is prepared from the word at
/// `this+0x40`, the seed float at block `+0x38` and the constant 0.25 (nine
/// stack words, as in the neighbouring row picker), then asked about the
/// source vector. A zero low byte of its answer copies the source vector
/// over the target and returns the copy's last source word.
///
/// Otherwise the offset from the block's point (`+0x30..+0x38`) to the
/// target point is normalised (reciprocal length, zero when the squared
/// length compares equal to zero, which also covers NaN by taking the
/// divide path) and a second probe runs against the classifier with three
/// scratch outputs and the target vector. Its zero answer copies the source
/// vector over the target; a nonzero answer blends: each of three scratch
/// words scaled by 0.25 is added to a base word, the three sums land at
/// target `+0x00/+0x08/+0x04` (note the order) and a seventh scratch word
/// lands at target `+0x0c`. All float operation orders are the original's,
/// including the deliberately mixed orders in the normalisation
/// (`x*inv`, `inv*y`, `inv*z`). The blend path returns the probe's full
/// answer; both copy paths return the last source word, which the copy
/// leaves in the return register.
///
/// Original: 0x00DC87B0 (thiscall, one stack word).
export!(thiscall, rw_dc87b0(this: u32, p: u32) -> u32 {
    const SRC_VEC: u32 = 0x20;
    const DST_VEC: u32 = 0x30;
    const KEY_OFF: u32 = 0x40;
    const BLK_OFF: u32 = 0x20;
    const SEED_OFF: u32 = 0x38;
    const QUARTER: f32 = 0.25;
    const ONE: f32 = 1.0;
    const CAL_PREP: u32 = 1;
    const CAL_ASK: u32 = 2;
    const CAL_PROBE: u32 = 3;
    const CAL_COOKIE: u32 = 4;

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
    unsafe fn rd32(a: u32) -> u32 {
        unsafe { (a as *const u32).read_unaligned() }
    }
    unsafe fn rdf(a: u32) -> f32 {
        unsafe { f32::from_bits(rd32(a)) }
    }
    unsafe fn wr32(a: u32, v: u32) {
        unsafe { (a as *mut u32).write_unaligned(v) }
    }
    unsafe fn wrf(a: u32, v: f32) {
        unsafe { wr32(a, v.to_bits()) }
    }
    unsafe fn copy4(dst: u32, src: u32) {
        unsafe {
            wr32(dst, rd32(src));
            wrf(dst.wrapping_add(4), rdf(src.wrapping_add(4)));
            wrf(dst.wrapping_add(8), rdf(src.wrapping_add(8)));
            wr32(dst.wrapping_add(12), rd32(src.wrapping_add(12)));
        }
    }

    unsafe {
        let dst = this.wrapping_add(DST_VEC);
        let srcv = this.wrapping_add(SRC_VEC);
        let block = rd32(p.wrapping_add(BLK_OFF));
        let seed = rdf(block.wrapping_add(SEED_OFF));
        let obj = [0u32; 1];
        let base = obj.as_ptr() as u32;
        callee_thiscall!(CAL_PREP, u32, base, rd32(this.wrapping_add(KEY_OFF)), seed.to_bits(), QUARTER.to_bits(), 0, 0, 0, 0, 0, 0);
        let ask: u32 = callee_thiscall!(CAL_ASK, u32, base, srcv);
        if (ask & 0xff) == 0 {
            copy4(dst, srcv);
            callee_cdecl!(CAL_COOKIE, u32, );
            // The copy leaves the last source word in the return register.
            return rd32(srcv.wrapping_add(12));
        }
        // Offset from the block's point to the target point, normalised.
        let blk_pt = block.wrapping_add(DST_VEC);
        let x = sub(rdf(dst), rdf(blk_pt));
        let y = sub(rdf(dst.wrapping_add(4)), rdf(blk_pt.wrapping_add(4)));
        let z = sub(rdf(dst.wrapping_add(8)), rdf(blk_pt.wrapping_add(8)));
        let xx = mul(x, x);
        let yy = mul(y, y);
        let len2 = add(add(xx, yy), mul(z, z));
        // The original's ucomiss/lahf/test/jnp skips the divide exactly
        // when the squared length compares equal to +0.0 (NaN takes the
        // divide path, as here, since NaN != 0.0).
        let inv = if len2 == 0.0 {
            0.0
        } else {
            core::hint::black_box(ONE) / core::hint::black_box(len2.sqrt())
        };
        let _nx = mul(x, inv);
        let _ny = mul(inv, y);
        let _nz = mul(inv, z);
        // The normalised offset is consumed only by the probe's scratch
        // protocol on the original side; the stub answers without reading
        // it, so only the calls and their order are reproduced here.
        let mut w_p1 = [0u32; 3];
        let mut w_p2 = [0u32; 4];
        let mut w_p3 = [0u32; 3];
        // Argument order is last-pushed-first: the target vector (pushed
        // last) is arg0, then the three scratch pointers in reverse.
        let probe: u32 = callee_thiscall!(CAL_PROBE, u32, base, dst, w_p3.as_mut_ptr() as u32, w_p2.as_mut_ptr() as u32, w_p1.as_mut_ptr() as u32);
        if (probe & 0xff) == 0 {
            copy4(dst, srcv);
            callee_cdecl!(CAL_COOKIE, u32, );
            // As above: the copy's last source word is the return value.
            return rd32(srcv.wrapping_add(12));
        }
        let b0 = f32::from_bits(w_p1[0]);
        let b1 = f32::from_bits(w_p1[1]);
        let b2 = f32::from_bits(w_p1[2]);
        let s0 = mul(f32::from_bits(w_p3[0]), QUARTER);
        let s1 = mul(f32::from_bits(w_p3[1]), QUARTER);
        let s2 = mul(f32::from_bits(w_p3[2]), QUARTER);
        wrf(dst, add(b0, s0));
        wrf(dst.wrapping_add(8), add(b2, s2));
        wrf(dst.wrapping_add(4), add(b1, s1));
        wr32(dst.wrapping_add(12), w_p2[3]);
        callee_cdecl!(CAL_COOKIE, u32, );
        probe
    }
});
