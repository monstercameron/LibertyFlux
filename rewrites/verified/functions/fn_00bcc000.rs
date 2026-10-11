// original: 0x00BCC000 physics_copy_vector_components
/// Copy four 32-bit component patterns from the resolved object's vector into
/// four caller-provided output slots. The cdecl inputs are a context handle
/// followed by four output pointers. A manager resolver returns an object
/// whose vector pointer is stored at byte offset 0x20. A thiscall helper fills
/// a four-word local buffer through ECX while receiving that vector pointer as
/// its stack argument; each resulting word is copied unchanged, preserving
/// every float bit pattern. The function's EAX result is the fourth output
/// pointer. This proof snapshots the local buffer at the helper call and
/// compares all four output writes.
lf_checker_rt::export!(
    cdecl,
    rw_00BCC000(
        context_handle: u32,
        x_out: u32,
        y_out: u32,
        z_out: u32,
        w_out: u32,
    ) -> u32 {
        const PHYSICS_MANAGER_SLOT: u32 = 0x012e_22a4;
        const VECTOR_ARRAY_POINTER_OFFSET: u32 = 0x20;
        const COMPONENT_COUNT: u32 = 4;
        let manager = unsafe {
            core::ptr::read_unaligned(lf_checker_rt::global::<u32>(PHYSICS_MANAGER_SLOT))
        };
        let object = lf_checker_rt::callee_thiscall!(1, u32, manager, context_handle);
        let vector = unsafe {
            core::ptr::read_unaligned((object + VECTOR_ARRAY_POINTER_OFFSET) as *const u32)
        };
        let mut components = [0u32; COMPONENT_COUNT as usize];
        let component_buffer = components.as_mut_ptr() as usize as u32;
        let _copied = lf_checker_rt::callee_thiscall!(2, u32, component_buffer, vector);
        let output_slots = [x_out, y_out, z_out, w_out];
        for index in 0..COMPONENT_COUNT {
            let bits = unsafe {
                core::ptr::read_unaligned((component_buffer + index * 4) as *const u32)
            };
            unsafe {
                core::ptr::write_unaligned(output_slots[index as usize] as *mut u32, bits);
            }
        }
        w_out
    }
);
