// original: 0x0094cf90 indexed_shape_contains_point
/// Test a point against a globally indexed handler, as a boolean.
///
/// Looks up the handler through the global index table, forwards the three
/// position words plus an extra float to it (with a trailing zero word), and
/// returns whether its answer was nonzero.
export!(stdcall, rw_0094cf90(pos: *const u32, extra: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0118D818;
        let index = *global::<u32>(TABLE);
        let handler = *global::<u32>(TABLE + index.wrapping_mul(4));
        let this = handler.wrapping_add(0x10);
        let answer = callee_thiscall!(1, u32, this, *pos, *pos.add(1), *pos.add(2), extra, 0u32);
        (answer != 0) as u32
    }
});
