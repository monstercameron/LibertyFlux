// original: 0x00926C30 ui_axis_view_constants_upload (proposed)
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global};

/// Upload the view constants of one effect record for one of six axis
/// orientations.
///
/// `record` selects a 0x110-byte record of the effect table and `axis`
/// (0..=5) selects an orientation: the high two bits of the value pick which
/// of three basis-vector triples of the record play the first, second and
/// third roles, and the low bit negates the third. The function uploads three
/// small vectors through three shared handles (the first carries the axis
/// number as a float, the others the record's scale and its inverse), builds
/// a 4x4 view matrix (three basis columns, with the negated dot products of
/// the offset position as the last row, and one unused word after each row),
/// has the matrix routine process it into a second matrix, builds an
/// identity matrix, combines the two through the combine routine and uploads
/// the result through the last handle.
///
/// For `axis` above 5 the original reads its nine basis floats from stack
/// memory it never wrote; this rewrite uses zero for them (the checker fills
/// that memory with zero). Returns nothing.
export!(cdecl, rw_926c30(record: i32, axis: u32) -> () {
    unsafe {
        const HANDLE_AXIS: u32 = 0x0119D050;
        const HANDLE_SCALE: u32 = 0x0119D044;
        const HANDLE_ORIGIN: u32 = 0x0119D04C;
        const HANDLE_VIEW: u32 = 0x0119D034;
        const TILT_GLOBAL: u32 = 0x0119D028;
        const RECORDS: u32 = 0x0119F000;
        const RECORD_STRIDE: i32 = 0x110;
        const FIELD_OFFSET_SCALE: u32 = 0x110;
        const FIELD_SIZE: u32 = 0x114;
        const BASIS_A: u32 = 0x140;
        const BASIS_B: u32 = 0x150;
        const BASIS_C: u32 = 0x160;
        const SHIFT: u32 = 0x170;
        const AXIS_COUNT: u32 = 6;

        let f32_at = |va: u32| global::<f32>(va).read();
        let u32_at = |va: u32| global::<u32>(va).read();
        let rec = RECORDS.wrapping_add(record.wrapping_mul(RECORD_STRIDE) as u32);
        let rf = |off: u32| f32_at(rec.wrapping_add(off));
        let triple = |base: u32| [rf(base), rf(base + 4), rf(base + 8)];

        // Three small uploads.
        let v = [3.0f32, (axis as i32) as f32, 0.0, 0.0];
        callee_cdecl!(1, u32, u32_at(HANDLE_AXIS), v.as_ptr() as u32);
        let tilt = f32_at(TILT_GLOBAL);
        let v = [0.2f32, 0.0, tilt, -1.0f32 / rf(FIELD_SIZE)];
        callee_cdecl!(2, u32, u32_at(HANDLE_SCALE), v.as_ptr() as u32);
        let v = [0.0f32, 0.0, 0.0, rf(FIELD_SIZE)];
        callee_cdecl!(3, u32, u32_at(HANDLE_ORIGIN), v.as_ptr() as u32);

        // Basis columns by orientation pair; the odd orientation negates the
        // third column.
        let (col0, col1, col2): ([f32; 3], [f32; 3], [f32; 3]) = if axis < AXIS_COUNT {
            let (p, q, r) = match axis >> 1 {
                0 => (BASIS_A, BASIS_B, BASIS_C),
                1 => (BASIS_C, BASIS_B, BASIS_A),
                _ => (BASIS_A, BASIS_C, BASIS_B),
            };
            let third = triple(r);
            let third = if axis & 1 != 0 {
                [-third[0], -third[1], -third[2]]
            } else {
                third
            };
            (triple(p), triple(q), third)
        } else {
            ([0.0; 3], [0.0; 3], [0.0; 3])
        };

        // Position: the third basis triple scaled by the negated record
        // factor, plus the record's shift vector.
        let neg_scale = -rf(FIELD_OFFSET_SCALE);
        let pos_x = rf(BASIS_C) * neg_scale + rf(SHIFT);
        let pos_y = rf(BASIS_C + 4) * neg_scale + rf(SHIFT + 4);
        let pos_z = rf(BASIS_C + 8) * neg_scale + rf(SHIFT + 8);
        // Negated dot product of the position with one basis column.
        let neg_dot = |c: &[f32; 3]| -> f32 { -((pos_z * c[2]) + ((pos_y * c[1]) + (pos_x * c[0]))) };

        // 4x4 view matrix, row by row, with one unused word per row.
        let mut view: [u32; 16] = [
            col0[0].to_bits(), col1[0].to_bits(), col2[0].to_bits(), 0,
            col0[1].to_bits(), col1[1].to_bits(), col2[1].to_bits(), 0,
            col0[2].to_bits(), col1[2].to_bits(), col2[2].to_bits(), 0,
            neg_dot(&col0).to_bits(), neg_dot(&col1).to_bits(), neg_dot(&col2).to_bits(), 0,
        ];
        let mut processed = [0u32; 16];
        callee_cdecl!(4, u32, processed.as_mut_ptr() as u32, view.as_ptr() as u32);

        // Identity matrix, combined with the processed matrix.
        view = [
            1.0f32.to_bits(), 0, 0, 0,
            0, 1.0f32.to_bits(), 0, 0,
            0, 0, 1.0f32.to_bits(), 0,
            0, 0, 0, 1.0f32.to_bits(),
        ];
        let mut combined = [0u32; 16];
        callee_thiscall!(5, u32, combined.as_mut_ptr() as u32,
            view.as_ptr() as u32, processed.as_ptr() as u32);
        callee_cdecl!(6, u32, u32_at(HANDLE_VIEW), combined.as_ptr() as u32);
    }
});
