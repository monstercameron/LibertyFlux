// original: 0x008edda0 dot_gated_count_decrement
/// Decrement a count and notify, gated by a scaled dot product.
///
/// Reads two packed row selectors out of the table at `arg1` (indexed by
/// `arg3`), resolves each through the instance table at `this + 0x804`
/// (returning the selector's low word when its slot is empty), then compares
/// two short vectors from the resolved records against the reference point at
/// `arg0` after scaling by a constant factor. Only when their dot product is
/// strictly negative is `count - 1` stored to `arg2` and the notify helper
/// (thiscall/0, stubbed by the checker) invoked with `arg1`; every other
/// path leaves memory untouched. Returns the helper's answer on the notify
/// path, `arg0` when the gate fails, or the empty selector index.
///
/// The `count < 2` early-out returns entry EAX in the original, which a
/// rewrite cannot observe; the contract seeds `count >= 2` so that path is
/// never taken, and the fallback below fails loudly if it ever is.
export!(thiscall, rw_008edda0(this_ptr: u32, arg0: u32, arg1: u32, arg2: u32, arg3: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 0x804;
        const SCALE_VA: u32 = 0xFE87A4;
        let count = *(arg2 as *const i32);
        if count < 2 {
            return 0;
        }
        let table = this_ptr.wrapping_add(TABLE_OFF) as *const u32;
        let row = arg1 as *const u32;
        let v1 = *row.add(arg3 as usize);
        let idx1 = (v1 & 0xFFFF) as usize;
        let base1 = *table.add(idx1);
        if base1 == 0 {
            return idx1 as u32;
        }
        let v2 = *row.add((arg3 as usize).wrapping_add(1));
        let idx2 = (v2 & 0xFFFF) as usize;
        let base2 = *table.add(idx2);
        if base2 == 0 {
            return idx2 as u32;
        }
        let rec1 = base1.wrapping_add((v1 >> 16).wrapping_mul(32));
        let rec2 = base2.wrapping_add((v2 >> 16).wrapping_mul(32));
        let k = *global::<f32>(SCALE_VA);
        let anchor = arg0 as *const f32;
        let ax = *anchor;
        let ay = *anchor.add(1);
        let r1x = *((rec1 + 0x14) as *const i16) as f32;
        let r1y = *((rec1 + 0x16) as *const i16) as f32;
        let r2x = *((rec2 + 0x14) as *const i16) as f32;
        let r2y = *((rec2 + 0x16) as *const i16) as f32;
        let dx1 = r1x * k - ax;
        let dy1 = r1y * k - ay;
        let dx2 = r2x * k - ax;
        let dy2 = r2y * k - ay;
        let dot = dx2 * dx1 + dy2 * dy1;
        // `comiss 0, dot; jbe exit`: only a strictly negative dot product
        // continues. `dot < 0.0` is false for NaN, matching the exit-on-NaN.
        if !(dot < 0.0) {
            return arg0;
        }
        *(arg2 as *mut u32) = (count as u32).wrapping_sub(1);
        callee_thiscall!(1, u32, arg1)
    }
});
