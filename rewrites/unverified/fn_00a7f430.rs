// original: 0x00A7F430 CComplexOpenVehicleDoorTaskInfo::vf6

/// Serialize the inherited die-task state, then conditionally serialize a
/// nested task through the object's virtual slot. `this` is in ECX and the
/// output stream pointer is the single stack argument. The inherited helper's
/// low return byte is the result byte; when the virtual predicate is zero,
/// that byte is returned directly. The nonzero path forwards the field at
/// `+0x18`, the virtual result, and a pointer to the local result byte to the
/// stream helper. This proof exercises the zero-predicate path only.
///
/// The virtual slot is at byte offset `+0x44` in the vtable. All calls use
/// scripted checker stubs, so this rewrite does not execute native callees.
lf_checker_rt::export!(thiscall, rw_00a7f430(this: u32, stream: u32) -> u32 {
    unsafe {
        const SERIALIZE_DIE: u32 = 1;
        const VIRTUAL_PREDICATE: u32 = 2;
        const SERIALIZE_EXTENSION: u32 = 3;
        const VIRTUAL_SLOT: u32 = 0x44;
        const TASK_FIELD: u32 = 0x18;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let result_byte = lf_checker_rt::callee_thiscall!(SERIALIZE_DIE, u32, this, stream) as u8;
        let vtable = read_u32(this);
        let predicate_address = read_u32(vtable.wrapping_add(VIRTUAL_SLOT));
        let predicate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(predicate_address as usize);
        let predicate_result = predicate(this);

        if predicate_result == 0 {
            return result_byte as u32;
        }

        let mut extension_flag = result_byte;
        let extension_flag_ptr = &mut extension_flag as *mut u8 as u32;
        let extension: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(predicate_address as usize);
        let extension_result = extension(this, extension_flag_ptr);
        let field_value = read_u32(this.wrapping_add(TASK_FIELD));
        let _ = lf_checker_rt::callee_thiscall!(
            SERIALIZE_EXTENSION,
            u32,
            stream,
            field_value,
            extension_result,
            extension_flag_ptr
        );
        extension_flag as u32
    }
});
