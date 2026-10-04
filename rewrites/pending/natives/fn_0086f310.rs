// original: 0x0086f310 VDIST
/// Script native `VDIST`.
///
/// Computes the Euclidean distance between two 3D points given as six
/// script floats (x1, y1, z1, x2, y2, z2) and stores the result into the
/// return slot. This handler inlines the arithmetic with SSE scalar ops
/// instead of calling an engine function; the summation order here matches
/// the original exactly (`(dy*dy + dx*dx) + dz*dz`, then square root), so
/// results agree bit-for-bit.
export!(cdecl, rw_0086f310(ctx: *const u8) -> u32 {
    unsafe {
        let args = (*(ctx.add(8) as *const u32)) as *const f32;
        let dx = *args - *args.add(3);
        let dy = *args.add(1) - *args.add(4);
        let dz = *args.add(2) - *args.add(5);
        let dist = (dy * dy + dx * dx + dz * dz).sqrt();
        let slot = *(ctx as *const u32) as *mut f32;
        *slot = dist;
        slot as u32
    }
});
