// original: 0x00d8e0e0 payandspray_compressor_setup
/// Set up the pay-and-spray compressor nuance for the entity.
///
/// Releases a stale nuance handle when one is attached, stages a scratch
/// block, resolves the source and its tag, and asks the shared builder to
/// assemble the compressor. When the builder declines, the source is torn
/// down instead. When it accepts, the fixed effect cells are registered and
/// the mixer is invoked. Returns the mixer's answer on the accept path and
/// the teardown's answer otherwise.
lf_rs89_rt::export!(thiscall, rw_00d8e0e0(this: *mut u8) -> u32 {
    unsafe {
        let gate = *(this.wrapping_add(0xC) as *const u32);
        if gate != 0 {
            lf_rs89_rt::callee_thiscall!(1, u32, gate, 0);
        }
        let mut stage = [0u32; 8];
        lf_rs89_rt::callee_thiscall!(2, u32, stage.as_mut_ptr() as u32);
        let src: u32 = lf_rs89_rt::callee_cdecl!(3, u32,);
        let tag: u32 = lf_rs89_rt::callee_cdecl!(4, u32, src);
        let mut block = [0u32; 4];
        let accepted: u32 = lf_rs89_rt::callee_thiscall!(
            5,
            u32,
            this as u32,
            lf_rs89_rt::relocated(0xEEDB30),
            this.wrapping_add(0xC) as u32,
            block.as_mut_ptr() as u32,
            src,
            tag,
            0
        );
        if accepted & 0xFF == 0 {
            return lf_rs89_rt::callee_cdecl!(8, u32, src);
        }
        let mut cells = [0u32, 0xFFFFFFFF, 0x15];
        let mut extra = [0u32; 4];
        let mixed: u32 = lf_rs89_rt::callee_cdecl!(
            6,
            u32,
            lf_rs89_rt::relocated(0xEEDB48),
            0,
            0,
            1,
            1,
            extra.as_mut_ptr() as u32,
            cells.as_mut_ptr() as u32,
            0
        );
        lf_rs89_rt::callee_cdecl!(7, u32, mixed)
    }
});
