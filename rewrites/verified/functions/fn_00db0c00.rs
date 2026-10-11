// original: 0x00DB0C00 mainloop_timing_clear_two_triples
/// Calls the shared three-word initializer for the receiver and its adjacent
/// record, then clears all six words. The calls are observable even though
/// their writes are superseded. The method takes no stack arguments and
/// returns the receiver in EAX.
lf_checker_rt::export!(thiscall, rw_00DB0C00(destination: *mut u32) -> *mut u32 {
    const WORDS_PER_TRIPLE: usize = 3;
    const TOTAL_WORDS: usize = WORDS_PER_TRIPLE * 2;

    let first_record = destination;
    let second_record = unsafe { destination.add(WORDS_PER_TRIPLE) };
    let first_receiver = first_record as usize as u32;
    let second_receiver = second_record as usize as u32;

    let _first_result = lf_checker_rt::callee_thiscall!(1, u32, first_receiver, 0u32);
    let _second_result = lf_checker_rt::callee_thiscall!(1, u32, second_receiver, 0u32);

    // SAFETY: the receiver points to two writable three-word records.
    unsafe { core::ptr::write_bytes(destination, 0, TOTAL_WORDS); }
    destination
});
