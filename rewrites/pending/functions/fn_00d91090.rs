// original: 0x00d91090 audio_cell_centroid
/// Average of one cell's vertex positions (centroid of its corners).
///
/// `this` points at the audio geometry object: +0x60 holds the packed
/// vertex-index array, +0x6c the cell table (40 bytes per cell). `index`
/// selects the cell; `out` receives the averaged position. Each cell stores
/// a corner count in bits 21..=24 of its first word and a base index into
/// the vertex array (masked to 17 bits) in its second word. Corner positions
/// are resolved one by one through the vertex-fetch callee, summed into
/// `out`, then divided by the count. A cell with no corners leaves `out` as
/// NaN (zero times an infinite reciprocal), matching the original exactly.
/// Returns the corner count (the value the original leaves in eax).
export!(thiscall, rw_00d91090(this: u32, index: u32, out: *mut f32) -> u32 {
    let cells = unsafe { *((this + 0x6c) as *const u32) };
    let entry = cells.wrapping_add(index.wrapping_mul(40));
    let count = (unsafe { *(entry as *const u32) } >> 0x15) & 0xf;
    // Reciprocal first: for an empty cell this is +inf, and the final
    // multiply below turns the zeroed accumulator into NaN, as observed.
    let inv = 1.0f32 / (count as f32);
    unsafe {
        *out = 0.0;
        *out.add(1) = 0.0;
        *out.add(2) = 0.0;
    }
    if count != 0 {
        let base = unsafe { *((entry + 4) as *const u32) } & 0x1ffff;
        let verts = unsafe { *((this + 0x60) as *const *const u16) };
        let mut i = 0u32;
        while i < count {
            let v = unsafe { *verts.add((base + i) as usize) } as u32;
            let mut corner = [0.0f32; 3];
            callee_thiscall!(1, u32, this, v, corner.as_mut_ptr() as u32);
            unsafe {
                *out += corner[0];
                *out.add(1) += corner[1];
                *out.add(2) += corner[2];
            }
            i += 1;
        }
    }
    unsafe {
        *out *= inv;
        *out.add(1) *= inv;
        *out.add(2) *= inv;
    }
    count
});
