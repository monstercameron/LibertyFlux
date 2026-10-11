// original: 0x005f34c0 CopyFragmentVectorsAndUpdate

/// Copies two four-word vector records into this fragment instance at `+0x260`
/// and `+0x270`, preserving each word's bits, then calls the instance update
/// helper and returns its result. The two source pointers are stack arguments.
lf_checker_rt::export!(thiscall, rw_005f34c0(this: u32, first_source: u32, second_source: u32) -> u32 {
    const FIRST_DESTINATION: u32 = 0x260;
    const SECOND_DESTINATION: u32 = 0x270;
    const WORD_OFFSETS: [u32; 4] = [0, 4, 8, 12];

    for offset in WORD_OFFSETS {
        let first_word = unsafe { (first_source.wrapping_add(offset) as *const u32).read_unaligned() };
        let second_word = unsafe { (second_source.wrapping_add(offset) as *const u32).read_unaligned() };
        unsafe {
            (this.wrapping_add(FIRST_DESTINATION).wrapping_add(offset) as *mut u32).write_unaligned(first_word);
            (this.wrapping_add(SECOND_DESTINATION).wrapping_add(offset) as *mut u32).write_unaligned(second_word);
        }
    }
    lf_checker_rt::callee_thiscall!(1, u32, this)
});
