// original: 0x00b35c70 frame_struct_call
/// Forward a float triple and a level to the blend helper as one record.
///
/// Packs the three floats at the incoming pointer, a padding word and the
/// incoming level into a five-word frame record and hands it to the helper
/// with the helper's table and the constant words 0, 0x16, 0x15. The
/// padding word is never written (it keeps whatever the stack held).
export!(cdecl, rw_00b35c70(triple: *const f32, level: f32) -> u32 {
    unsafe {
        let record = [
            *triple,
            *triple.add(1),
            *triple.add(2),
            0.0,
            level,
        ];
        callee_cdecl!(
            1,
            u32,
            record.as_ptr() as u32,
            relocated(0x00b3bf40),
            0,
            0x16,
            0x15
        );
        0
    }
});
