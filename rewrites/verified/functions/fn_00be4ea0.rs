// original: 0x00BE4EA0 map_task_slots_16

/// Read sixteen pointer entries after a one-word header. For each non-null
/// child, call its vtable slot `+4` and store the returned word into the
/// corresponding output slot at `this+4`; for a null child, store zero.
/// Return the last child result, or the original input-array pointer if no
/// child was non-null, matching the original's live accumulator.
///
/// The thiscall method receives the input-array pointer as one stack word.
/// Child objects and their vtables are modeled with fabricated heap segments,
/// so the checker records each indirect call. The empty-entry store is the
/// deliberately perturbed behavior in the mutant.
lf_checker_rt::export!(thiscall, rw_00be4ea0(this: u32, input_array: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        let mut return_value = input_array;
        for index in 0..16u32 {
            let child = read_u32(input_array + 4 + index * 4);
            if child == 0 {
                write_u32(this + 4 + index * 4, 0);
            } else {
                let vtable = read_u32(child);
                let getter_address = read_u32(vtable + 4);
                let getter: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(getter_address as usize);
                return_value = getter(child);
                write_u32(this + 4 + index * 4, return_value);
            }
        }
        return_value
    }
});
