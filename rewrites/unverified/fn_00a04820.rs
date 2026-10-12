// original: 0x00A04820 NativeImpl_GET_OBJECT_QUATERNION
/// Looks up the object for its handle, reads twelve raw float words from the
/// object's transform pointer at +0x20, and copies them into the corresponding
/// positions of an 80-byte local matrix buffer. It calls the quaternion helper
/// with the buffer base and its +0x10 subview, then copies the helper-written
/// first four words to the caller's four output pointers.
lf_checker_rt::export!(cdecl, rw_00A04820(handle: u32, out_x: u32, out_y: u32, out_z: u32, out_w: u32) -> () {
    unsafe {
        const OBJECT_POOL: u32 = 0x0163_2C60;
        const OBJECT_TRANSFORM: u32 = 0x20;
        const BUFFER_WORDS: usize = 20;
        const MATRIX_FIELDS: [u32; 12] = [0x00, 0x04, 0x08, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];

        let pool = lf_checker_rt::global::<u32>(OBJECT_POOL).read_volatile();
        let object = lf_checker_rt::callee_thiscall!(1, u32, pool, handle);
        let source = (object.wrapping_add(OBJECT_TRANSFORM) as *const u32).read_unaligned();
        let mut buffer = core::mem::MaybeUninit::<[u32; BUFFER_WORDS]>::uninit();
        let buffer_base = buffer.as_mut_ptr() as *mut u32;
        for source_offset in MATRIX_FIELDS {
            let destination_word = (source_offset + 0x10) / 4;
            let word = (source.wrapping_add(source_offset) as *const u32).read_unaligned();
            buffer_base.add(destination_word as usize).write_unaligned(word);
        }

        let buffer_address = buffer_base as u32;
        let _ = lf_checker_rt::callee_thiscall!(2, u32, buffer_address, buffer_address.wrapping_add(0x10));
        let result = buffer_base as *const u32;
        let outputs = [out_x, out_y, out_z, out_w];
        for (index, destination) in outputs.into_iter().enumerate() {
            let bits = result.add(index).read_unaligned();
            (destination as *mut u32).write_unaligned(bits);
        }
    }
});
