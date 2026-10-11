// original: 0x006A28B0 allocate_zeroed_stream_table
/// Allocates storage for `entry_count` eight-byte records and clears it.
///
/// The allocator object is read from the current thread's TLS context and its
/// allocation method is called through the object's function table with the
/// byte count, allocation type, and zero flags. The allocation call happens
/// for every signed input, including zero and negative counts. Only a positive
/// count triggers the clearing loop; a null allocation skips the stores while
/// preserving the original loop and return behavior. The function uses
/// stdcall with one signed stack argument.
const TLS_CONTEXT_SLOT_28B0: usize = 0;
const ALLOCATOR_CONTEXT_OFFSET_WORDS: usize = 2;
const CONTEXT_VTABLE_OFFSET_WORDS: usize = 0;
const VTABLE_ALLOCATE_SLOT_WORDS: usize = 2;
const RECORD_BYTES: u32 = 8;
const ALLOCATION_KIND: u32 = 0x10;

lf_checker_rt::export!(stdcall, rw_006a28b0(entry_count: i32) -> u32 {
    let tls_context = lf_checker_rt::tls_slot(TLS_CONTEXT_SLOT_28B0);
    let allocator_context = unsafe {
        core::ptr::read_volatile(
            (tls_context as *const u32).add(ALLOCATOR_CONTEXT_OFFSET_WORDS),
        )
    };
    let allocator_vtable = unsafe {
        core::ptr::read_volatile(
            (allocator_context as *const u32).add(CONTEXT_VTABLE_OFFSET_WORDS),
        )
    };
    let allocate = unsafe {
        core::ptr::read_volatile(
            (allocator_vtable as *const u32).add(VTABLE_ALLOCATE_SLOT_WORDS),
        )
    };
    let allocate_fn: unsafe extern "thiscall" fn(u32, u32, u32, u32) -> u32 = unsafe {
        core::mem::transmute(allocate as usize)
    };
    let byte_count = (entry_count as u32).wrapping_mul(RECORD_BYTES);
    let allocation = unsafe { allocate_fn(allocator_context, byte_count, ALLOCATION_KIND, 0) };

    if entry_count > 0 {
        let mut index = 0i32;
        let mut record = allocation;
        while index < entry_count {
            if record != 0 {
                let record_words = record as *mut u32;
                unsafe {
                    core::ptr::write_volatile(record_words, 0);
                    core::ptr::write_volatile(record_words.add(1), 0);
                }
            }
            record = record.wrapping_add(RECORD_BYTES);
            index += 1;
        }
    }

    allocation
});
