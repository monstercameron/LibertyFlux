// original: 0x00639DC0 rage::ptxRule::init_from_type_name

/// Select behavior from the NUL-terminated inline name at `this + 0x128`.
/// The sprite name calls its copy helper and returns that helper's EAX; the
/// model name resets the rule, changes the object's vtable, runs two model
/// helpers and returns the final helper's EAX. Any other name returns one.
/// `this` arrives in ECX and the secondary input arrives in EDX; the function
/// has no stack arguments. Its image strings and the written vtable are loaded
/// through relocated image references.
lf_checker_rt::export!(fastcall, rw_00639DC0(this: u32, secondary_input: u32) -> u32 {
    const INLINE_NAME: u32 = 0x128;
    const MODEL_HELPER_OBJECT: u32 = 0x168;
    const MODEL_HELPER_STATE: u32 = 0x174;
    const SPRITE_NAME: u32 = 0x00F98894;
    const MODEL_NAME: u32 = 0x00F988A0;
    const MODEL_VTABLE: u32 = 0x00FE282C;
    const COPY_SPRITE: u32 = 1;
    const RESET_RULE: u32 = 2;
    const LOAD_MODEL_PROPERTIES: u32 = 3;
    const PARSE_MODEL_PROPERTIES: u32 = 4;

    let matches_image_name = |image_address: u32| {
        let mut object_byte = this.wrapping_add(INLINE_NAME);
        let mut image_byte = lf_checker_rt::relocated(image_address);
        loop {
            let object_value = unsafe { (object_byte as *const u8).read() };
            let image_value = unsafe { (image_byte as *const u8).read() };
            if object_value != image_value {
                break false;
            }
            if object_value == 0 {
                break true;
            }
            object_byte = object_byte.wrapping_add(1);
            image_byte = image_byte.wrapping_add(1);
        }
    };

    if matches_image_name(SPRITE_NAME) {
        if this != 0 {
            return lf_checker_rt::callee_thiscall!(COPY_SPRITE, u32, this, secondary_input);
        }
        return 0;
    }

    if !matches_image_name(MODEL_NAME) {
        return 1;
    }
    if this == 0 {
        return 0;
    }

    let _ = lf_checker_rt::callee_thiscall!(RESET_RULE, u32, this, secondary_input);
    unsafe {
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(MODEL_VTABLE));
    }
    let _ = lf_checker_rt::callee_thiscall!(
        LOAD_MODEL_PROPERTIES,
        u32,
        this.wrapping_add(MODEL_HELPER_OBJECT),
        secondary_input,
        this,
    );
    lf_checker_rt::callee_thiscall!(
        PARSE_MODEL_PROPERTIES,
        u32,
        this.wrapping_add(MODEL_HELPER_STATE),
        secondary_input,
    )
});
