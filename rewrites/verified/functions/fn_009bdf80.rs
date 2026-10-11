// original: 0x009BDF80 CEventImminentDanger::CEventImminentDanger
/// Constructs an imminent-danger event with the 32-bit MSVC thiscall ABI.
/// It calls the base constructor, installs the event vtable, copies two
/// three-word vectors and a scalar f32 bit pattern into the event fields,
/// stores the remaining words and low flag byte, then retains a nonnull child
/// through the slot at offset 0x44. It returns the event pointer.
lf_checker_rt::export!(thiscall, rw_009bdf80(
    this: u32,
    first_vector: u32,
    scalar_bits: u32,
    second_vector: u32,
    event_kind: u32,
    child: u32,
    category: u32,
    flag: u32,
) -> u32 {
    const EVENT_VTABLE_VA: u32 = 0x00E9_4CDC;
    const FIRST_VECTOR_OFFSET: u32 = 0x10;
    const SCALAR_BITS_OFFSET: u32 = 0x20;
    const SECOND_VECTOR_OFFSET: u32 = 0x30;
    const EVENT_KIND_OFFSET: u32 = 0x40;
    const CHILD_SLOT_OFFSET: u32 = 0x44;
    const CATEGORY_OFFSET: u32 = 0x48;
    const FLAG_OFFSET: u32 = 0x4C;

    let _ = lf_checker_rt::callee_thiscall!(1, u32, this);
    unsafe { (this as *mut u32).write(lf_checker_rt::relocated(EVENT_VTABLE_VA)) };

    for word_index in 0..3u32 {
        let displacement = word_index * 4;
        let first_word = unsafe { ((first_vector + displacement) as *const u32).read() };
        let second_word = unsafe { ((second_vector + displacement) as *const u32).read() };
        unsafe {
            ((this + FIRST_VECTOR_OFFSET + displacement) as *mut u32).write(first_word);
            ((this + SECOND_VECTOR_OFFSET + displacement) as *mut u32).write(second_word);
        }
    }
    unsafe {
        ((this + SCALAR_BITS_OFFSET) as *mut u32).write(scalar_bits);
        ((this + EVENT_KIND_OFFSET) as *mut u32).write(event_kind);
        ((this + CHILD_SLOT_OFFSET) as *mut u32).write(child);
        ((this + CATEGORY_OFFSET) as *mut u32).write(category);
        ((this + FLAG_OFFSET) as *mut u8).write(flag as u8);
    }

    if child != 0 {
        let child_slot = this.wrapping_add(CHILD_SLOT_OFFSET);
        let _ = lf_checker_rt::callee_thiscall!(2, u32, child, child_slot);
    }
    this
});
