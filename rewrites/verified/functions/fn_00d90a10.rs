// original: 0x00d90a10 audio_publish_point_bounds
/// Publish a point's expanded bounds and forward to the cell search.
///
/// `v` is a 4-float vector and `f` a radius. Writes the point minus/plus
/// the radius, the raw point, and two zero words into the shared bound
/// globals, packs the scaled-down (x8, truncated) bound corners into six
/// words, and calls the cell-search routine with the child-table head
/// (`this`+0x70), the packed bounds and the opaque `tag`. The original
/// reads one uninitialized stack slot for the two zero words; the checker
/// contract defines that slot as zero. Returns whatever the search returns.
export!(thiscall, rw_00d90a10(this: u32, v: *const f32, f: f32, tag: u32) -> u32 {
    let d = unsafe { [*v, *v.add(1), *v.add(2), *v.add(3)] };
    // Uninitialized stack slot in the original; defined as zero by contract.
    let pad = 0.0f32;
    let lo = [d[0] - f, d[1] - f, d[2] - f];
    let hi = [fadd(d[0], f), fadd(d[1], f), fadd(d[2], f)];
    unsafe {
        *global::<f32>(0x179fafc) = pad;
        *global::<f32>(0x179faf0) = lo[0];
        *global::<f32>(0x179faf4) = lo[1];
        *global::<f32>(0x179faf8) = lo[2];
        *global::<f32>(0x179fb0c) = pad;
        *global::<f32>(0x179fb00) = hi[0];
        *global::<f32>(0x179fb04) = hi[1];
        *global::<f32>(0x179fb08) = hi[2];
        *global::<f32>(0x179fb10) = d[0];
        *global::<f32>(0x179fb14) = d[1];
        *global::<f32>(0x179fb18) = d[2];
        *global::<f32>(0x179fb1c) = d[3];
    }
    let bounds = [
        cvtt(lo[0] * 8.0) as u16,
        cvtt(hi[0] * 8.0) as u16,
        cvtt(lo[1] * 8.0) as u16,
        cvtt(hi[1] * 8.0) as u16,
        cvtt(lo[2] * 8.0) as u16,
        cvtt(hi[2] * 8.0) as u16,
    ];
    let head = unsafe { *((this + 0x70) as *const u32) };
    callee_thiscall!(1, u32, this, head, bounds.as_ptr() as u32, tag)
});
