// original: 0x0086CF00 scrThread::scrThread

/// Initialize the base script-thread object and allocate its two buffers.
///
/// The thiscall object pointer is in ECX. The constructor installs the
/// relocated script-thread vtable, initializes its allocator fields, and
/// obtains the allocator object from TLS slot zero. It calls vtable slot two
/// twice with sizes 0x2004 and 0x2000, stores the returned pointers at byte
/// offsets 0x60 and 0x68, and makes the two cdecl clear calls in order. It
/// then sets the documented sentinel fields and returns the original object
/// pointer in EAX. The intercepted clear helper's writes are modeled by its
/// own proof, not by this constructor rewrite.
lf_checker_rt::export!(thiscall, rw_0086CF00(context: u32) -> u32 {
    const THREAD_VTABLE_VA: u32 = 0x00FE_7EB0;
    const THREAD_AUXILIARY_VA: u32 = 0x00FC_7210;
    const ALLOCATED_SCRIPT_BUFFER_OFFSET: usize = 0x60 / 4;
    const ALLOCATED_TASK_BUFFER_OFFSET: usize = 0x68 / 4;
    const ALLOCATOR_OBJECT_OFFSET_WORDS: usize = 2;
    const ALLOCATOR_METHOD_SLOT_WORDS: usize = 2;
    const CLEAR_HELPER: u32 = 2;
    const ALLOCATOR_POOL_FLAGS: u32 = 0x10;
    const SCRIPT_BUFFER_BYTES: u32 = 0x2004;
    const TASK_BUFFER_BYTES: u32 = 0x2000;

    unsafe {
        let object = context as *mut u32;
        object.write_unaligned(lf_checker_rt::global::<u32>(THREAD_VTABLE_VA) as u32);
        object.add(0x58 / 4).write_unaligned(0);
        object.add(0x5C / 4).write_unaligned(0);
        object.add(ALLOCATED_SCRIPT_BUFFER_OFFSET).write_unaligned(0);
        object.add(0x64 / 4).write_unaligned(0);
        object.add(ALLOCATED_TASK_BUFFER_OFFSET).write_unaligned(0);
        object.add(0x6C / 4).write_unaligned(
            lf_checker_rt::global::<u32>(THREAD_AUXILIARY_VA) as u32,
        );

        let tls_block = lf_checker_rt::tls_slot(0);
        let allocator_object = (tls_block as *const u32)
            .add(ALLOCATOR_OBJECT_OFFSET_WORDS)
            .read_unaligned();
        let allocator_vtable = (allocator_object as *const u32).read_unaligned();
        let allocate_address = (allocator_vtable as *const u32)
            .add(ALLOCATOR_METHOD_SLOT_WORDS)
            .read_unaligned();
        let allocate: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(allocate_address as usize);

        let script_buffer = allocate(
            allocator_object,
            SCRIPT_BUFFER_BYTES,
            ALLOCATOR_POOL_FLAGS,
            0,
        );
        object.add(ALLOCATED_SCRIPT_BUFFER_OFFSET)
            .write_unaligned(script_buffer);
        let _ = lf_checker_rt::callee_cdecl!(
            CLEAR_HELPER,
            u32,
            script_buffer,
            0,
            SCRIPT_BUFFER_BYTES
        );

        let task_buffer = allocate(
            allocator_object,
            TASK_BUFFER_BYTES,
            ALLOCATOR_POOL_FLAGS,
            0,
        );
        object.add(ALLOCATED_TASK_BUFFER_OFFSET)
            .write_unaligned(task_buffer);
        let _ = lf_checker_rt::callee_cdecl!(
            CLEAR_HELPER,
            u32,
            context.wrapping_add(4),
            0,
            0x54
        );

        object.add(0x2C / 4).write_unaligned(u32::MAX);
        object.add(0x30 / 4).write_unaligned(u32::MAX);
        object.add(0x0C / 4).write_unaligned(0);
        object.add(0x44 / 4).write_unaligned(0);
        object.add(0x54 / 4).write_unaligned(u32::MAX);
        context
    }
});
