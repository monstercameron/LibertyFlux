// original: 0x00A86A50 release_and_destroy_slot_object
/// Decrements the slot count, clears the saved object pointer when it matches
/// the argument, and destroys a non-null object through virtual slot zero.
/// The byte return is one on every completing path.
///
/// The counter structure stores its count at offset 0 and saved object at
/// offset 0x0C. A non-null object begins with a vtable pointer; the selected
/// entry receives the object in ECX and the destroy flag 1 on the stack.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_a86a50(counter: u32, object: u32) -> u32 {
    const COUNT_OFFSET_WORDS: usize = 0;
    const SAVED_OBJECT_OFFSET_WORDS: usize = 3;
    const DESTROY_FLAG: u32 = 1;

    let counter_words = counter as *mut u32;
    let count = unsafe { counter_words.add(COUNT_OFFSET_WORDS).read_unaligned() };
    unsafe { counter_words.add(COUNT_OFFSET_WORDS).write_unaligned(count.wrapping_sub(1)); }
    let saved_object = unsafe { counter_words.add(SAVED_OBJECT_OFFSET_WORDS).read_unaligned() };
    if object == saved_object {
        unsafe { counter_words.add(SAVED_OBJECT_OFFSET_WORDS).write_unaligned(0); }
    }
    if object != 0 {
        lf_checker_rt::callee_thiscall!(1, u32, object, DESTROY_FLAG);
    }
    1
}
