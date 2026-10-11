// original: 0x00dc83a0 task_ctor_pair_positions_ref

/// Construct a task object holding two copies of a three-word position and a
/// reference pointer. The base constructor receives the first stack word as
/// raw `f32` bits. The object stores relocated vtable values at `+0` and
/// `+0x14`, copies the source vector to `+0x20` and `+0x30`, and stores the
/// reference at `+0x40`; a non-null reference is passed with the slot address
/// to the reference helper. This thiscall routine cleans three stack words.
lf_checker_rt::export!(thiscall, rw_00dc83a0(this_ptr: u32, weight_bits: u32, source_ptr: u32, reference: u32) -> u32 {
    unsafe {
        const PRIMARY_VTABLE_VA: u32 = 0x00ef4eac;
        const SECONDARY_VTABLE_VA: u32 = 0x00ef4f04;
        const FIRST_POSITION: u32 = 0x20;
        const SECOND_POSITION: u32 = 0x30;
        const REFERENCE_SLOT: u32 = 0x40;
        const BASE_CTOR: u32 = 1;
        const REFERENCE_HELPER: u32 = 2;
        let _base_result = lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this_ptr, weight_bits);
        (this_ptr as *mut u32).write_unaligned(lf_checker_rt::relocated(PRIMARY_VTABLE_VA));
        (this_ptr.wrapping_add(0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(SECONDARY_VTABLE_VA));
        for (source_offset, destination_offset) in [
            (0x00u32, FIRST_POSITION), (0x04, FIRST_POSITION + 4), (0x08, FIRST_POSITION + 8),
            (0x00, SECOND_POSITION), (0x04, SECOND_POSITION + 4), (0x08, SECOND_POSITION + 8),
        ] {
            let value = (source_ptr.wrapping_add(source_offset) as *const u32).read_unaligned();
            (this_ptr.wrapping_add(destination_offset) as *mut u32).write_unaligned(value);
        }
        (this_ptr.wrapping_add(REFERENCE_SLOT) as *mut u32).write_unaligned(reference);
        if reference != 0 {
            let slot = this_ptr.wrapping_add(REFERENCE_SLOT);
            let _helper_result = lf_checker_rt::callee_thiscall!(REFERENCE_HELPER, u32, reference, slot);
        }
        this_ptr
    }
});
