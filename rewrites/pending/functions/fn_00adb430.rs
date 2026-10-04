// original: 0x00adb430 grid_cell_interp
/// Interpolate a grid value at a point inside a cell, gated by range checks.
///
/// Takes a descriptor with four cell-corner indices plus a flag word, two
/// coordinates, a reference value, an output pointer, two tolerance values
/// and an optional flag-output pointer. Each coordinate must fall between
/// the edge values the corner indices select from a shared table; the two
/// normalized positions must sum to at most one for the lower-triangle path
/// (otherwise the upper-triangle path runs). The interpolated table value is
/// stored through the output pointer, then must pass two ordered gates
/// against the reference and tolerances, with the descriptor's bit 2 acting
/// as an extra reject switch on one gate. Any unordered (NaN) range
/// comparison fails its gate. Returns 1 with the flag bit written on
/// success, 0 otherwise.
export!(cdecl, rw_00adb430(obj: u32, a1: f32, a2: f32, a3: f32, out: u32, a5: f32, a6: f32, bout: u32) -> u32 {
    // Bit-exact scalar add/sub with the hardware's NaN selection
    // (destination NaN wins quieted, else source NaN, else the result):
    // LLVM commutes plain `+` at will, which swaps NaN payloads.
    fn addss(dest: f32, src: f32) -> f32 {
        if dest.is_nan() {
            return f32::from_bits(dest.to_bits() | 0x00400000);
        }
        if src.is_nan() {
            return f32::from_bits(src.to_bits() | 0x00400000);
        }
        dest + src
    }
    fn subss(dest: f32, src: f32) -> f32 {
        if dest.is_nan() {
            return f32::from_bits(dest.to_bits() | 0x00400000);
        }
        if src.is_nan() {
            return f32::from_bits(src.to_bits() | 0x00400000);
        }
        dest - src
    }
    unsafe {
        const TABLE: u32 = 0x0158E860;
        const ONE_CELL: u32 = 0x00FE88E8;

        let table = relocated(TABLE);
        let row = |idx: i32| table.wrapping_add((idx << 3) as u32);
        let word_at = |addr: u32| (addr as *const i16).read_unaligned() as i32;
        let float_at = |addr: u32| (addr as *const f32).read_unaligned();

        let ia = ((obj.wrapping_add(4)) as *const i16).read_unaligned() as i32;
        let base_a = row(ia);
        let lo_x = word_at(base_a) as f32;
        if !(a1 >= lo_x) {
            return 0;
        }
        let ib = ((obj.wrapping_add(6)) as *const i16).read_unaligned() as i32;
        let hi_x = word_at(row(ib)) as f32;
        if !(hi_x >= a1) {
            return 0;
        }
        let lo_y = word_at(base_a.wrapping_add(2)) as f32;
        if !(a2 >= lo_y) {
            return 0;
        }
        let jb = ((obj.wrapping_add(8)) as *const i16).read_unaligned() as i32;
        let hi_y = word_at(row(jb).wrapping_add(2)) as f32;
        if !(hi_y >= a2) {
            return 0;
        }
        let dx = word_at(row(ib)).wrapping_sub(word_at(base_a)) as f32;
        let tx = (a1 - lo_x) / dx;
        let dy = word_at(row(jb).wrapping_add(2))
            .wrapping_sub(word_at(base_a.wrapping_add(2))) as f32;
        let ty = (a2 - lo_y) / dy;
        let one = (relocated(ONE_CELL) as *const f32).read_unaligned();
        let t = addss(ty, tx);

        let flags = ((obj.wrapping_add(0x0c)) as *const u16).read_unaligned();
        let r = if one >= t {
            let v0 = float_at(base_a.wrapping_add(4));
            let v1 = float_at(row(ib).wrapping_add(4));
            let v2 = float_at(row(jb).wrapping_add(4));
            addss(addss((v1 - v0) * tx, v0), (v2 - v0) * ty)
        } else {
            let jc = ((obj.wrapping_add(0x0a)) as *const i16).read_unaligned() as i32;
            let vb = float_at(row(jb).wrapping_add(4));
            let vc = float_at(row(jc).wrapping_add(4));
            let vd = float_at(row(ib).wrapping_add(4));
            addss(
                addss((vb - vc) * (one - tx), vc),
                (vd - vc) * (one - ty),
            )
        };
        (out as *mut f32).write_unaligned(r);
        let d = subss(r, a5);
        if d > a3 {
            if flags & 4 != 0 {
                return 0;
            }
        }
        if a3 > addss(r, a6) {
            return 0;
        }
        if bout != 0 {
            (bout as *mut u8).write_unaligned(((flags & 8) >> 3) as u8);
        }
        1
    }
});
