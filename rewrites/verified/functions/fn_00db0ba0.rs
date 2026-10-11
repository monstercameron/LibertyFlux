// original: 0x00DB0BA0 mainloop_timing_initialize_two_triples
/// Initializes two adjacent three-word timing values. The `thiscall` receiver
/// is the first record and the two stack arguments supply the leading `f32`
/// word in each record. It calls the shared three-word initializer for each
/// record in order, then stores the exact input bits and returns the receiver.
lf_checker_rt::export!(thiscall, rw_00DB0BA0(destination: *mut u32, first: f32, second: f32) -> *mut u32 {
    const WORDS_PER_TRIPLE: usize = 3;
    const FIRST_VALUE_WORD: usize = 0;
    const SECOND_VALUE_WORD: usize = WORDS_PER_TRIPLE;

    let first_record = destination;
    let second_record = unsafe { destination.add(SECOND_VALUE_WORD) };
    let first_receiver = first_record as usize as u32;
    let second_receiver = second_record as usize as u32;

    let _first_result = lf_checker_rt::callee_thiscall!(1, u32, first_receiver, 0u32);
    let _second_result = lf_checker_rt::callee_thiscall!(1, u32, second_receiver, 0u32);

    let fields = destination;
    // SAFETY: the receiver points to two adjacent three-word records.
    unsafe {
        fields.add(FIRST_VALUE_WORD).write(first.to_bits());
        fields.add(FIRST_VALUE_WORD + 1).write(0);
        fields.add(FIRST_VALUE_WORD + 2).write(0);
        fields.add(SECOND_VALUE_WORD).write(second.to_bits());
        fields.add(SECOND_VALUE_WORD + 1).write(0);
        fields.add(SECOND_VALUE_WORD + 2).write(0);
    }
    destination
});
