// original: 0x00DCCE20 CTaskComplexMoveAvoidOtherPedWhileWandering::vf9
/// Copies one four-word motion record from the second stack argument into the
/// task's fields at offsets 0x2c through 0x38. The record's first and last
/// words are copied as integers; the two middle words preserve their exact
/// floating-point bit patterns. The original is thiscall and pops three
/// stack arguments, although only the second argument is read.
lf_checker_rt::export!(thiscall, rw_00dcce20(
    task: u32,
    _unused_first: u32,
    source_record: u32,
    _unused_third: u32
) -> () {
    const RECORD_WORDS: usize = 4;
    const DESTINATION_OFFSET: usize = 0x2c;

    // SAFETY: the checker contract supplies a readable four-word source and
    // a writable task object. The game layout is four consecutive dwords.
    unsafe {
        core::ptr::copy_nonoverlapping(
            source_record as *const u32,
            (task as *mut u8).add(DESTINATION_OFFSET) as *mut u32,
            RECORD_WORDS,
        );
    }
});
