// original: 0x00CCEFB0 CTaskComplexMelee::CTaskComplexMelee
const VTABLE: u32 = 0x00EDA884;
const BASE_CTOR: u32 = 1;
const RESET_SUBOBJECT: u32 = 2;
const SET_TARGET_NODE: u32 = 3;

const FIELD_VTABLE: u32 = 0x00;
const FIELD_INITIAL_WORD: u32 = 0x30;
const FIELD_FIRST_IMAGE_POINTER: u32 = 0x38;
const FIELD_SECOND_IMAGE_POINTER: u32 = 0x3C;
const FIELD_READY_FLAG: u32 = 0x40;
const FIELD_FLAGS: u32 = 0xF0;
const FIELD_FLAGS_HIGH: u32 = 0xF1;

#[inline(always)]
unsafe fn write_word(object: u32, offset: u32, value: u32) {
    // SAFETY: the caller provides the constructor's writable object storage.
    unsafe { (object.wrapping_add(offset) as *mut u32).write_unaligned(value) }
}

#[inline(always)]
unsafe fn write_short(object: u32, offset: u32, value: u16) {
    // SAFETY: the caller provides the constructor's writable object storage.
    unsafe { (object.wrapping_add(offset) as *mut u16).write_unaligned(value) }
}

#[inline(always)]
unsafe fn write_byte(object: u32, offset: u32, value: u8) {
    // SAFETY: the caller provides the constructor's writable object storage.
    unsafe { (object.wrapping_add(offset) as *mut u8).write_unaligned(value) }
}

#[inline(always)]
unsafe fn read_byte(object: u32, offset: u32) -> u8 {
    // SAFETY: the caller provides the constructor's readable object storage.
    unsafe { (object.wrapping_add(offset) as *const u8).read_unaligned() }
}

/// Constructs the melee task object. It calls the base constructor, installs
/// its vtable, initializes task and flag fields, resets ten embedded helper
/// slots, forwards the last two arguments to the target-node setter, and
/// returns `this`. The first stack word is stored at `+0x30`; the low bits of
/// stack words two through five form the low flag byte at `+0xF0`. The last
/// two stack words are forwarded in their original parameter order.
#[unsafe(no_mangle)]
pub extern "thiscall" fn rw_ctask_complex_melee_ctor(
    this_ptr: u32,
    initial_word: u32,
    flag_two: u32,
    flag_three: u32,
    flag_four: u32,
    flag_five: u32,
    target_first: u32,
    target_second: u32,
) -> u32 {
    let _ = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this_ptr);

    unsafe {
        write_word(this_ptr, FIELD_VTABLE, lf_checker_rt::relocated(VTABLE));
        write_word(this_ptr, 0x20, 0);
        write_word(this_ptr, 0x24, 0);
        write_word(this_ptr, 0x28, 0);
        write_word(this_ptr, FIELD_INITIAL_WORD, initial_word);
        write_word(this_ptr, 0x34, 0);
        write_word(this_ptr, FIELD_FIRST_IMAGE_POINTER, 0);
        write_word(this_ptr, FIELD_SECOND_IMAGE_POINTER, 0);
        write_short(this_ptr, 0x40, 0);
        write_word(this_ptr, 0x44, 0);
        write_word(this_ptr, 0x48, 0);
        write_short(this_ptr, 0x4C, 0);
        write_word(this_ptr, 0x50, 0);

        let _ = lf_checker_rt::callee_thiscall!(RESET_SUBOBJECT, u32, this_ptr.wrapping_add(0x54));
        let mut slot = 0;
        while slot <= 6 {
            lf_checker_rt::callee_thiscall!(
                RESET_SUBOBJECT,
                u32,
                this_ptr.wrapping_add(0x60 + slot * 0x0C),
            );
            slot += 1;
        }
        let _ = lf_checker_rt::callee_thiscall!(RESET_SUBOBJECT, u32, this_ptr.wrapping_add(0xB4));
        let _ = lf_checker_rt::callee_thiscall!(RESET_SUBOBJECT, u32, this_ptr.wrapping_add(0xC0));

        for offset in [0xCC, 0xD0, 0xD4, 0xD8, 0xDC, 0xE0] {
            write_word(this_ptr, offset, 0);
        }
        write_short(this_ptr, 0xE4, 0);
        let flags = 0x80
            | ((flag_five as u8 & 1) << 3)
            | ((flag_three as u8 & 1) << 2)
            | ((flag_two as u8 & 1) << 1)
            | (flag_four as u8 & 1);
        write_byte(this_ptr, FIELD_FLAGS, flags);
        let prior_high_flags = read_byte(this_ptr, FIELD_FLAGS_HIGH);
        write_byte(this_ptr, FIELD_FLAGS_HIGH, prior_high_flags & 0xF0);
        write_word(this_ptr, 0xE8, 0);
        write_word(this_ptr, 0xEC, 0);

        lf_checker_rt::callee_thiscall!(
            SET_TARGET_NODE,
            u32,
            this_ptr,
            target_first,
            target_second,
        );

        let first_global = lf_checker_rt::global::<u32>(0x011735B4).read_unaligned();
        let second_global = lf_checker_rt::global::<u32>(0x010519BC).read_unaligned();
        write_word(this_ptr, FIELD_FIRST_IMAGE_POINTER, first_global);
        write_word(this_ptr, FIELD_SECOND_IMAGE_POINTER, second_global);
        write_byte(this_ptr, FIELD_READY_FLAG, 1);
    }

    this_ptr
}
