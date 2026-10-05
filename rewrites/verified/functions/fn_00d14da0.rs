// original: 0x00d14da0 combat_range_local_offset_update (proposed)
//
// Re-expresses a target offset into a local frame and refreshes the four
// registered consumer lists.
//
// `this` is a combat-state object. It holds a world position at +0x13d0
// (three floats) and three local-offset slots at +0x13e0. `frame` points at
// a 3x3 row-major rotation plus a position: rows at +0x00/+0x10/+0x20 (three
// floats each) and the frame position at +0x30.
//
// The rewrite first forms the displacement `d = this.pos - frame.pos`, then
// stores one dot product per matrix row (`row.y*dy + row.x*dx + row.z*dz`,
// in the original's accumulation order) into the local-offset slots. It then
// calls the offset-consumer hook (callee 1, no stack arguments) and walks
// four consumer lists. List `k` has a signed count at +0x13a0+4k, one flag
// byte per entry starting at its flag base, and entries of fixed stride
// starting at its object base; every entry whose flag byte is non-zero is
// passed (with `frame`) to that list's callee:
//
// | list | count   | flags | objects | stride | callee |
// | 0    | +0x13a0 | +0x13b0 | +0x90  | 0xb0   | 2 |
// | 1    | +0x13a4 | +0x13bb | +0x820 | 0xc0   | 3 |
// | 2    | +0x13a8 | +0x13bc | +0x8e0 | 0xe0   | 4 |
// | 3    | +0x13ac | +0x13c7 | +0x1280 | 0x120 | 5 |
//
// A non-positive count skips its list. The returned value is the leftover in
// the accumulator: the end pointer of the last entered list (object base +
// count * stride), the list-0 count when only list 0 ran, or zero when no
// list ran. All callees are thiscall and take `frame` as their stack
// argument. Original convention: thiscall (this in ECX, one stack word).
lf_checker_rt::export!(thiscall, rw_00d14da0(this: u32, frame: u32) -> u32 {
    unsafe {
        const POS_X: u32 = 0x13d0;
        const POS_Y: u32 = 0x13d4;
        const POS_Z: u32 = 0x13d8;
        const OUT_X: u32 = 0x13e0;
        const OUT_Y: u32 = 0x13e4;
        const OUT_Z: u32 = 0x13e8;
        const COUNT_BASE: u32 = 0x13a0;
        const FLAG_BASES: [u32; 4] = [0x13b0, 0x13bb, 0x13bc, 0x13c7];
        const OBJ_BASES: [u32; 4] = [0x90, 0x820, 0x8e0, 0x1280];
        const STRIDES: [u32; 4] = [0xb0, 0xc0, 0xe0, 0x120];
        const HOOK_CALLEE: u32 = 1;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
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
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
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

        let dx = sub(rdf(this + POS_X), rdf(frame + 0x30));
        let dy = sub(rdf(this + POS_Y), rdf(frame + 0x34));
        let dz = sub(rdf(this + POS_Z), rdf(frame + 0x38));
        // Row 0: (ry*dy + rx*dx) + rz*dz.
        let t1 = mul(rdf(frame + 4), dy);
        let t0 = mul(rdf(frame), dx);
        let t1 = add(t1, t0);
        let t0 = mul(rdf(frame + 8), dz);
        wrf(this + OUT_X, add(t1, t0));
        // Row 1.
        let t0 = mul(dx, rdf(frame + 0x10));
        let t1 = mul(rdf(frame + 0x14), dy);
        let t1 = add(t1, t0);
        let t0 = mul(rdf(frame + 0x18), dz);
        wrf(this + OUT_Y, add(t1, t0));
        // Row 2: t1=ry, t4=dx*rx, t0=rz, t1=t1*dy, t0=t0*dz, t1+=t4, out=t1+t0.
        let t1 = rdf(frame + 0x24);
        let t4 = mul(dx, rdf(frame + 0x20));
        let t0 = rdf(frame + 0x28);
        let t1 = mul(t1, dy);
        let t0 = mul(t0, dz);
        let t1 = add(t1, t4);
        wrf(this + OUT_Z, add(t1, t0));

        let _: u32 = lf_checker_rt::callee_thiscall!(HOOK_CALLEE, u32, this);

        let mut ret: u32 = 0;
        let mut k = 0u32;
        while k < 4 {
            let count = rd32(this + COUNT_BASE + k * 4) as i32;
            if count > 0 {
                let mut i = 0i32;
                while i < count {
                    if rd8(this + FLAG_BASES[k as usize] + i as u32) != 0 {
                        let obj = this + OBJ_BASES[k as usize] + (i as u32) * STRIDES[k as usize];
                        let id = k + 2;
                        // One call site per list so the call log matches.
                        if id == 2 {
                            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, obj, frame);
                        } else if id == 3 {
                            let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, obj, frame);
                        } else if id == 4 {
                            let _: u32 = lf_checker_rt::callee_thiscall!(4, u32, obj, frame);
                        } else {
                            let _: u32 = lf_checker_rt::callee_thiscall!(5, u32, obj, frame);
                        }
                    }
                    i += 1;
                }
                if k == 0 {
                    ret = count as u32;
                } else {
                    ret = this.wrapping_add(OBJ_BASES[k as usize])
                        + (count as u32) * STRIDES[k as usize];
                }
            }
            k += 1;
        }
        ret
    }
});
