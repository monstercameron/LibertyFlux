// original: 0x00a99030 transform_four_indexed_vec3
/// Transforms four indexed vertex records into world-space vectors and
/// forwards them to a consumer.
///
/// Same shape as its three-record sibling: `st` points to a parameter block
/// with per-axis scale/bias pairs, an origin, and a pointer to a packed
/// table of signed 16-bit triplets. `i2`..`i5` select four records, each is
/// converted to float, scaled and biased axis by axis, and the four
/// resulting vectors are passed by pointer to the consumer: the four
/// indices in forward order first, then the vector pointers in reverse
/// order. `a0` is forwarded opaquely. Returns the consumer's result.
export!(cdecl, rw_a99030(a0: u32, st: u32, i2: u32, i3: u32, i4: u32, i5: u32) -> u32 {
    unsafe {
        let sp = st as *const f32;
        let bx: f32 = sp.add(36).read_unaligned();
        let by: f32 = sp.add(37).read_unaligned();
        let bz: f32 = sp.add(38).read_unaligned();
        let oy: f32 = sp.add(40).read_unaligned();
        let ox: f32 = sp.add(41).read_unaligned();
        let oz: f32 = sp.add(42).read_unaligned();
        let tab: u32 = (st as *const u32).add(44).read_unaligned();
        // x86 addss propagates the FIRST NaN operand's payload, but LLVM
        // commutes base + product (seen in this DLL's disassembly), which
        // changes the payload when both are NaN. Force the original's order
        // explicitly: a NaN base wins outright, otherwise a plain add is
        // order-free (at most the product is NaN, and it propagates the
        // same from either side).
        let add_o = |a: f32, b: f32| -> f32 {
            if a.is_nan() {
                f32::from_bits(a.to_bits() | 0x0040_0000)
            } else {
                a + b
            }
        };
        let downward = |idx: u32| -> [f32; 3] {
            let off: u32 = idx.wrapping_mul(3).wrapping_mul(2);
            let p = (tab.wrapping_add(off)) as *const i16;
            let sx = p.read_unaligned() as f32;
            let sy = p.add(1).read_unaligned() as f32;
            let sz = p.add(2).read_unaligned() as f32;
            [add_o(oy, bx * sx), add_o(ox, by * sy), add_o(oz, bz * sz)]
        };
        let va = downward(i5);
        let vb = downward(i4);
        let vc = downward(i3);
        let vd = downward(i2);
        lf_checker_rt::callee_thiscall!(
            1, u32, a0,
            i2, i3, i4, i5,
            vd.as_ptr() as u32, vc.as_ptr() as u32, vb.as_ptr() as u32, va.as_ptr() as u32
        )
    }
});
