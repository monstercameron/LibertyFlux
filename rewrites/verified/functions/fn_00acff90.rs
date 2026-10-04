// original: 0x00acff90 accumulate_scaled_projection
/// Accumulate a scaled projection of one vector onto another.
///
/// Takes a coefficient triple (at 0xd0 above the first pointer) and an input
/// triple, forms their dot product, scales it by a shared constant, then adds
/// each coefficient times that scaled value onto the triple at 0xb0 above the
/// object. Returns the input pointer, as the original leaves it in place.
export!(thiscall, rw_00acff90(this: *mut u8, coef: *const u8, input: *const u8) -> u32 {
    unsafe {
        let x = *(coef.add(0xd0) as *const f32);
        let y = *(coef.add(0xd4) as *const f32);
        let z = *(coef.add(0xd8) as *const f32);
        let ix = *(input as *const f32);
        let iy = *(input.add(4) as *const f32);
        let iz = *(input.add(8) as *const f32);
        // Exact accumulation order of the original: ((iy*y + ix*x) + iz*z).
        // NaN-exact helpers: the three lanes are independent, which is what
        // lets the vectorizer swap operands and break NaN payload order.
        let v = fmul(
            fadd(fadd(fmul(iy, y), fmul(ix, x)), fmul(iz, z)),
            *global::<f32>(0x00FE8734),
        );
        // Each store adds the product onto memory: (c*v) + old.
        let acc = this.add(0xb0) as *mut f32;
        *acc = fadd(fmul(x, v), *acc);
        *acc.add(1) = fadd(fmul(y, v), *acc.add(1));
        *acc.add(2) = fadd(fmul(z, v), *acc.add(2));
        input as u32
    }
});

// The helpers below are part of this rewrite: they enforce
// destination-side NaN propagation that the vectorizer
// would otherwise break (see report). They are identical
// to the verified crate source.
/// Bit-exact scalar `a * b` with SSE destination-side NaN propagation.
///
/// Plain `a * b` is fine on its own, but LLVM's SLP vectorizer packs parallel
/// scalar multiplies into one packed multiply and may swap the lanes'
/// operands. For two NaNs with different payloads that picks the wrong
/// payload, so the NaN cases are decided explicitly and only non-NaN
/// operands (whose product is order-independent) reach the operator.
#[inline]
fn fmul(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        a
    } else if b.is_nan() {
        b
    } else {
        a * b
    }
}

/// Bit-exact scalar `a + b` with SSE destination-side NaN propagation.
///
/// Same hazard as [`fmul`]: packed adds swap operands, so NaN payloads are
/// decided explicitly. Non-NaN sums (including infinities) are
/// order-independent.
#[inline]
fn fadd(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        a
    } else if b.is_nan() {
        b
    } else {
        a + b
    }
}
