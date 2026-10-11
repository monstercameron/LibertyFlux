// original: 0x00dc8330 task_ctor_vector_with_reference

/// Construct a task object from a three-word vector and a reference pointer.
/// The object receives a relocated vtable value, the vector at `+0x20`, three
/// zeroed words at `+0x30`, the reference slot at `+0x40`, and two byte flags
/// at `+0x44..+0x46`. A non-null reference is passed to the reference helper
/// with the address of the object's slot. This thiscall routine cleans four
/// stack words and returns the object pointer.
lf_checker_rt::export!(thiscall, rw_00dc8330(this_ptr: u32, reference: u32, source_ptr: u32, first_flag: u32, second_flag: u32) -> u32 {
    unsafe {
        const CLASS_VTABLE_VA: u32 = 0x00ef4d6c;
        const SOURCE_X: u32 = 0x00;
        const SOURCE_Y: u32 = 0x04;
        const SOURCE_Z: u32 = 0x08;
        const POSITION_X: u32 = 0x20;
        const POSITION_Y: u32 = 0x24;
        const POSITION_Z: u32 = 0x28;
        const ZEROED_A: u32 = 0x30;
        const ZEROED_B: u32 = 0x34;
        const ZEROED_C: u32 = 0x38;
        const REFERENCE_SLOT: u32 = 0x40;
        const FLAG_A: u32 = 0x44;
        const FLAG_B: u32 = 0x45;
        const TERMINATOR: u32 = 0x46;
        const BASE_CTOR: u32 = 1;
        const REFERENCE_HELPER: u32 = 2;

        let _base_result = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this_ptr);
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(CLASS_VTABLE_VA));
        for (source_offset, destination_offset) in [
            (SOURCE_X, POSITION_X), (SOURCE_Y, POSITION_Y), (SOURCE_Z, POSITION_Z),
        ] {
            let value = (source_ptr.wrapping_add(source_offset) as *const u32).read_unaligned();
            (this_ptr.wrapping_add(destination_offset) as *mut u32).write_unaligned(value);
        }
        for offset in [ZEROED_A, ZEROED_B, ZEROED_C] {
            (this_ptr.wrapping_add(offset) as *mut u32).write_unaligned(0);
        }
        (this_ptr.wrapping_add(REFERENCE_SLOT) as *mut u32).write_unaligned(reference);
        (this_ptr.wrapping_add(FLAG_A) as *mut u8).write(first_flag as u8);
        (this_ptr.wrapping_add(FLAG_B) as *mut u8).write(second_flag as u8);
        (this_ptr.wrapping_add(TERMINATOR) as *mut u8).write(0);
        if reference != 0 {
            let slot = this_ptr.wrapping_add(REFERENCE_SLOT);
            let _helper_result = lf_checker_rt::callee_thiscall!(REFERENCE_HELPER, u32, reference, slot);
        }
        this_ptr
    }
});
