// original: 0x00D9EF70 copy_stream_record
/// Copies seven words from object offset 0x58 into the supplied destination through the base copy helper, then returns the destination pointer.
lf_checker_rt::export!(thiscall, rw_s675_00d9ef70(this: u32, destination: u32) -> u32 {
    const SOURCE_OFFSET: u32 = 0x58;
    let source = this.wrapping_add(SOURCE_OFFSET);
    let _: u32 = lf_checker_rt::callee_thiscall!(1, u32, destination, source);
    destination
});
