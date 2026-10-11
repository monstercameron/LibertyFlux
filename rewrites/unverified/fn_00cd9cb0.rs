// original: 0x00CD9CB0 CTaskSimpleBlendFromNM::CTaskSimpleBlendFromNM

/// Initialize a `CTaskSimpleBlendFromNM` object after its base constructor
/// and common NM-task initializer. The first two stack words are copied to
/// offsets 0x30 and 0x34, and the fixed field at 0x24 is set to 9. A non-null
/// entity pointer is stored at 0x50 and registered with the reference helper.
/// A non-null blend descriptor is copied as four raw words to offsets
/// 0x60 through 0x6C, preserving the exact bit patterns of its two floats.
/// The derived vtable is installed and `this` is returned.
///
/// The descriptor pointer is argument four; its words are at offsets 0, 4,
/// 8 and 0xC. The reference helper receives the entity in ECX and the address
/// of the field at `this + 0x50` as its stack argument.
///
/// Calling convention: thiscall with four 32-bit stack words. The base and
/// common initializers take no stack arguments; the reference helper is
/// thiscall with one stack pointer argument.
lf_checker_rt::export!(thiscall, rw_00cd9cb0(this: u32, first_value: u32, second_value: u32, entity: u32, descriptor: u32) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const COMMON_INITIALIZER: u32 = 2;
    const REGISTER_REFERENCE: u32 = 3;
    const FIRST_FIELD: u32 = 0x30;
    const SECOND_FIELD: u32 = 0x34;
    const TYPE_FIELD: u32 = 0x24;
    const ENTITY_FIELD: u32 = 0x50;
    const DESCRIPTOR_FIELD: u32 = 0x60;
    const DESCRIPTOR_WORDS: u32 = 4;
    const VTABLE: u32 = 0x00ED_D68C;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        let _ = lf_checker_rt::callee_thiscall!(COMMON_INITIALIZER, u32, this);
        ((this + FIRST_FIELD) as *mut u32).write_unaligned(first_value);
        ((this + SECOND_FIELD) as *mut u32).write_unaligned(second_value);
        ((this + TYPE_FIELD) as *mut u32).write_unaligned(9);

        if entity != 0 {
            ((this + ENTITY_FIELD) as *mut u32).write_unaligned(entity);
            let registration_slot = this + ENTITY_FIELD;
            let _ = lf_checker_rt::callee_thiscall!(
                REGISTER_REFERENCE, u32, entity, registration_slot
            );
        }

        if descriptor != 0 {
            for word_index in 0..DESCRIPTOR_WORDS {
                let bits = ((descriptor + word_index * 4) as *const u32).read_unaligned();
                ((this + DESCRIPTOR_FIELD + word_index * 4) as *mut u32)
                    .write_unaligned(bits);
            }
        }
        this
    }
});
