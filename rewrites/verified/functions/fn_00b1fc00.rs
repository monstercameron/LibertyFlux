// original: 0x00b1fc00 cell_coords_if_inside (proposed)

/// Stores masked cell coordinates when the point is inside the cell.
///
/// Thiscall with six stack words: three coordinates then three
/// out-pointers (callee pops 24 bytes). The cell origin is three dwords
/// at +0x18250 in the object; the point must lie within +32 in x and y
/// and +8 in z (signed compares). When inside, stores x and y masked to
/// 5 bits and z masked to 3 bits, and returns 1; otherwise stores
/// nothing and returns 0. Only AL is meaningful on return.
lf_checker_rt::export!(thiscall, rw_00b1fc00(this: u32, x: u32, y: u32, z: u32, outx: u32, outy: u32, outz: u32) -> u32 {
    unsafe {
        const ORIGIN: u32 = 0x18250;
        const EXTENT_XY: i32 = 0x20;
        const EXTENT_Z: i32 = 8;
        const MASK5: u32 = 0x1f;
        const MASK3: u32 = 0x07;
        let ox = ((this + ORIGIN) as *const i32).read_unaligned();
        let oy = ((this + ORIGIN + 4) as *const i32).read_unaligned();
        let oz = ((this + ORIGIN + 8) as *const i32).read_unaligned();
        let xi = x as i32;
        let yi = y as i32;
        let zi = z as i32;
        let inside = xi >= ox
            && xi.wrapping_sub(ox) < EXTENT_XY
            && yi >= oy
            && yi.wrapping_sub(oy) < EXTENT_XY
            && zi >= oz
            && zi.wrapping_sub(oz) < EXTENT_Z;
        if !inside {
            return 0;
        }
        (outx as *mut u32).write_unaligned(x & MASK5);
        (outy as *mut u32).write_unaligned(y & MASK5);
        (outz as *mut u32).write_unaligned(z & MASK3);
        1
    }
});
