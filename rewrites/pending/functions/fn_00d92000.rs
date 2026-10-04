// original: 0x00d92000 audio_format_and_emit
/// Format the entity's label into a scratch buffer and emit it.
///
/// Renders at most 256 bytes through the shared formatter (fixed format,
/// the entity id and tag), hands the text to the emitter, and returns the
/// emitter-guard's answer. The stack-cookie check the original performs
/// around its frame is a build artifact, not behaviour, and is verified as
/// an intercepted call like the rest.
lf_rs89_rt::export!(cdecl, rw_00d92000(ent: u32, tag: u32) -> u32 {
    unsafe {
        let mut buf = [0u32; 64];
        let bp = buf.as_mut_ptr() as u32;
        lf_rs89_rt::callee_cdecl!(
            1,
            u32,
            bp,
            0x100,
            ent,
            tag,
            0,
            lf_rs89_rt::relocated(0xEEDE1C)
        );
        lf_rs89_rt::callee_cdecl!(2, u32, bp.wrapping_add(8));
        lf_rs89_rt::callee_cdecl!(3, u32,)
    }
});
