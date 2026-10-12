// original: 0x00685E60 crFrameBuffer::alloc_dof

/// Initialize the DOF subobject stored at `frame_buffer + 0x18`, ask its
/// virtual allocator for a byte advance, add that result to the frame
/// buffer's subobject pointer, and return the original subobject pointer.
/// The first call is a thiscall taking the supplied allocation kind; the
/// second is a zero-stack-argument thiscall through vtable slot two. The
/// object and vtable offsets are 32-bit engine layout fields.
lf_checker_rt::export!(thiscall, rw_00685E60(frame_buffer: u32, allocation_kind: u32) -> u32 {
    unsafe {
        const SUBOBJECT_PTR: u32 = 0x18;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let subobject = unsafe { read_u32(frame_buffer.wrapping_add(SUBOBJECT_PTR)) };
        let _ = unsafe { lf_checker_rt::callee_thiscall!(1, u32, subobject, allocation_kind) };
        let byte_advance = unsafe { lf_checker_rt::callee_thiscall!(2, u32, subobject) };
        unsafe {
            write_u32(
                frame_buffer.wrapping_add(SUBOBJECT_PTR),
                subobject.wrapping_add(byte_advance),
            )
        };
        subobject
    }
});
