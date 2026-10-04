// original: 0x00e5f320 fanout_float_global
/// Copy the shared float setting into this unit's two global slots.
///
/// A single 32-bit load followed by two stores; the bits are copied exactly,
/// so all values including NaNs survive unchanged.
export!(cdecl, rw_00e5f320() -> u32 {
    unsafe {
        const SRC: u32 = 0x17AD148;
        const DST_A: u32 = 0x1BB38A0;
        const DST_B: u32 = 0x1BB38A4;
        let v = (global::<f32>(SRC)).read();
        (global::<f32>(DST_A)).write(v);
        (global::<f32>(DST_B)).write(v);
        0
    }
});
