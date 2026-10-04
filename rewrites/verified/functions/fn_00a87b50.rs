// original: 0x00A87B50 reflect_rows_and_emit_normal
/// Reflect three row-vectors across a plane and emit their negated cross product.
///
/// Reads a plane (normal + distance) and three row-vectors out of the object,
/// reflects each row (the first against the offset plane, the other two against
/// the plane through the origin), then hands the negated cross product of the
/// last two reflections to the object's sink slot. Returns the sink's answer.
///
/// The sink's fourth word is whatever the scratch stack holds (the original
/// reads a slot it never wrote); under the checker's defined stack fill both
/// sides observe the fill value.
export!(thiscall, rw_a87b50(this: u32, plane: u32) -> u32 {
    const DOUBLE: f32 = 2.0;
    const SIGN_BITS: u32 = 0x8000_0000;
    const SINK_SLOT: u32 = 0xB0;
    // Row offsets inside the object.
    const R0: u32 = 0x120;
    const R1: u32 = 0x100;
    const R2: u32 = 0x110;

    // Uninitialized scratch word, matching the original's read of a stack slot
    // it never wrote. The worker fills scratch with the defined fill on both
    // sides, so both sides observe the same value.
    let pad = core::mem::MaybeUninit::<u32>::uninit();
    let pad_word = unsafe { pad.as_ptr().read_volatile() };

    // Row 0 and the plane distance feed only the dead row-0 reflection (the
    // original stores it to scratch and never reads it back); loaded here to
    // document the layout, then unused.
    let _r0 = unsafe {
        (
            ((this + R0) as *const f32).read(),
            ((this + R0 + 4) as *const f32).read(),
            ((this + R0 + 8) as *const f32).read(),
            (plane as *const f32).add(3).read(),
        )
    };
    let (n0, n1, n2) = unsafe {
        (
            (plane as *const f32).read(),
            (plane as *const f32).add(1).read(),
            (plane as *const f32).add(2).read(),
        )
    };
    let row = |base: u32| unsafe {
        (
            ((this + base) as *const f32).read(),
            ((this + base + 4) as *const f32).read(),
            ((this + base + 8) as *const f32).read(),
        )
    };
    // Every multiply and add goes through the opaque single-op helpers in
    // the original's operand order, so NaN payloads propagate identically.
    // Subtraction keeps its order naturally (it is not commutable).
    let reflect = |(vx, vy, vz): (f32, f32, f32)| {
        let t = ssadd(ssadd(ssmul(vy, n1), ssmul(vx, n0)), ssmul(vz, n2));
        let s = ssmul(t, DOUBLE);
        (vx - ssmul(n0, s), vy - ssmul(n1, s), vz - ssmul(n2, s))
    };
    let (bx, by, bz) = reflect(row(R1));
    let (cx, cy, cz) = reflect(row(R2));
    // Negated cross product of the reflected rows 2 and 1; negation is an
    // integer sign-bit flip, exactly the original's xorps with -0.0.
    let px = ssmul(cy, bz) - ssmul(cz, by);
    let py = ssmul(cz, bx) - ssmul(cx, bz);
    let pz = ssmul(cx, by) - ssmul(cy, bx);
    let frame = [
        px.to_bits() ^ SIGN_BITS,
        py.to_bits() ^ SIGN_BITS,
        pz.to_bits() ^ SIGN_BITS,
        pad_word,
    ];
    callee_thiscall!(1, u32, this.wrapping_add(SINK_SLOT), frame.as_ptr() as u32)
});

/// Scalar f32 multiply with pinned operand order: a single operation in an
/// opaque call, so the backend emits one \mulss\ with \\ as the
/// destination. A plain \ * b\ inside a larger function lets LLVM commute
/// the operands or pack rows into vector instructions, which combines NaN
/// operands in a different order than the original and changes NaN payloads.
#[inline(never)]
fn ssmul(a: f32, b: f32) -> f32 {
    a * b
}

/// Scalar f32 add with pinned operand order, as above.
#[inline(never)]
fn ssadd(a: f32, b: f32) -> f32 {
    a + b
}

