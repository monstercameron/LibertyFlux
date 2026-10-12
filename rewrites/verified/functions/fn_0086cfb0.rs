// original: 0x0086CFB0 rage::scrThread::~scrThread
/// Release each non-null thread buffer through the allocator in thread-local
/// storage, clear its object field, optionally release the thread object when
/// the low deletion flag is set, and return the object. The thread object and
/// allocator use the game's 32-bit layout; buffers are stored at byte offsets
/// 0x60 and 0x68, and the allocator's free method is virtual slot three.
lf_checker_rt::export!(thiscall, rw_0086CFB0(context: u32, delete_flag: u32) -> u32 {
    const THREAD_VTABLE_VA: u32 = 0x00FE_7EB0;
    const FIRST_BUFFER_OFFSET_WORDS: usize = 0x60 / 4;
    const SECOND_BUFFER_OFFSET_WORDS: usize = 0x68 / 4;
    const ALLOCATOR_TLS_OBJECT_WORDS: usize = 2;
    const FREE_METHOD_SLOT_WORDS: usize = 3;

    unsafe {
        let object = context as *mut u32;
        object.write_unaligned(lf_checker_rt::global::<u32>(THREAD_VTABLE_VA) as u32);
        let tls_block = lf_checker_rt::tls_slot(0);
        let allocator_object = (tls_block as *const u32)
            .add(ALLOCATOR_TLS_OBJECT_WORDS)
            .read_unaligned();
        let allocator_vtable = (allocator_object as *const u32).read_unaligned();
        let free_address = (allocator_vtable as *const u32)
            .add(FREE_METHOD_SLOT_WORDS)
            .read_unaligned();
        let free_allocation: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(free_address as usize);

        let first_buffer = object.add(FIRST_BUFFER_OFFSET_WORDS).read_unaligned();
        if first_buffer != 0 {
            let _ = free_allocation(allocator_object, first_buffer);
            object.add(FIRST_BUFFER_OFFSET_WORDS).write_unaligned(0);
        }
        let second_buffer = object.add(SECOND_BUFFER_OFFSET_WORDS).read_unaligned();
        if second_buffer != 0 {
            let _ = free_allocation(allocator_object, second_buffer);
            object.add(SECOND_BUFFER_OFFSET_WORDS).write_unaligned(0);
        }
        if delete_flag & 1 != 0 {
            let _ = free_allocation(allocator_object, context);
        }
    }
    context
});
