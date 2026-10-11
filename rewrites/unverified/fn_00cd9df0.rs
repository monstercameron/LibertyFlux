// original: 0x00CD9DF0 CTaskSimpleNMBalance::CTaskSimpleNMBalance

/// Initialize the extended `CTaskSimpleNMBalance` object after delegating its
/// first two arguments to the `CTaskSimpleNM` constructor and initializing a
/// nested blend-from-NM subobject. The derived vtable and flag byte at 0x57
/// are set, argument eight is stored at 0xE0, and argument three is stored
/// at 0x28 and registered when non-null. Argument four is stored at 0xC0 only
/// when argument seven is zero. The XYZ-like four-word payload at argument
/// five is copied to 0x30 through 0x3C when non-null; the first three fields
/// are cleared before that optional copy. Argument six's exact float bits are stored at 0x50, then
/// the halfword at 0x54 and byte at 0x56 are cleared. The function returns
/// `this`.
///
/// The seventh argument is also passed to the nested initializer at `this +
/// 0x60`; a non-null third-argument pointer is passed in ECX to the reference
/// helper together with the address of the field at `this + 0x28`. Argument
/// five points to four words, including raw float values at offsets 4 and 8.
///
/// Calling convention: thiscall with eight 32-bit stack words. The delegated
/// NM constructor takes two stack words, the nested initializer takes one,
/// and the reference helper takes one.
lf_checker_rt::export!(thiscall, rw_00cd9df0(this: u32, base_kind: u32, base_mode: u32, entity: u32, optional_value: u32, payload: u32, weight_bits: u32, nested_kind: u32, result_code: u32) -> u32 {
    const BASE_CONSTRUCTOR: u32 = 1;
    const INITIALIZE_NESTED: u32 = 2;
    const REGISTER_REFERENCE: u32 = 3;
    const NESTED_SUBOBJECT: u32 = 0x60;
    const ENTITY_FIELD: u32 = 0x28;
    const OPTIONAL_FIELD: u32 = 0xC0;
    const PAYLOAD_FIELD: u32 = 0x30;
    const PAYLOAD_WORDS: u32 = 4;
    const WEIGHT_FIELD: u32 = 0x50;
    const RESERVED_HALFWORD: u32 = 0x54;
    const RESERVED_BYTE: u32 = 0x56;
    const FLAGS: u32 = 0x57;
    const RESULT_FIELD: u32 = 0xE0;
    const VTABLE: u32 = 0x00ED_D2CC;

    unsafe {
        let _ = lf_checker_rt::callee_thiscall!(BASE_CONSTRUCTOR, u32, this, base_kind, base_mode);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        ((this + FLAGS) as *mut u8).write(0);
        let _ = lf_checker_rt::callee_thiscall!(
            INITIALIZE_NESTED, u32, this + NESTED_SUBOBJECT, nested_kind
        );
        ((this + RESULT_FIELD) as *mut u32).write_unaligned(result_code);
        ((this + ENTITY_FIELD) as *mut u32).write_unaligned(entity);

        if entity != 0 {
            let entity_slot = this + ENTITY_FIELD;
            let _ = lf_checker_rt::callee_thiscall!(
                REGISTER_REFERENCE, u32, entity, entity_slot
            );
        }
        if nested_kind == 0 {
            ((this + OPTIONAL_FIELD) as *mut u32).write_unaligned(optional_value);
        }

        ((this + PAYLOAD_FIELD) as *mut u32).write_unaligned(0);
        ((this + PAYLOAD_FIELD + 4) as *mut u32).write_unaligned(0);
        ((this + PAYLOAD_FIELD + 8) as *mut u32).write_unaligned(0);
        if payload != 0 {
            for word_index in 0..PAYLOAD_WORDS {
                let bits = ((payload + word_index * 4) as *const u32).read_unaligned();
                ((this + PAYLOAD_FIELD + word_index * 4) as *mut u32).write_unaligned(bits);
            }
        }
        ((this + WEIGHT_FIELD) as *mut u32).write_unaligned(weight_bits);
        ((this + RESERVED_HALFWORD) as *mut u16).write_unaligned(0);
        ((this + RESERVED_BYTE) as *mut u8).write(0);
        this
    }
});
