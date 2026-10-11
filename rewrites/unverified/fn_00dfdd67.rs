// original: 0x00DFDD67 heap_alloc
/// Allocate a heap block, lazily creating the process heap when needed.
///
/// A zero-byte request is promoted to one byte. Requests above the CRT
/// allocation limit and failed allocations set the thread error slot to
/// ENOMEM; retryable failures ask the CRT retry helper before returning.
const HEAP_HANDLE_FILE_VA: u32 = 0x17AC2B4;
const HEAP_RETRY_STATE_FILE_VA: u32 = 0x17AC3D8;
const MAX_HEAP_REQUEST: u32 = 0xFFFF_FFE0;
const ERROR_OUT_OF_MEMORY: u32 = 12;

unsafe fn set_out_of_memory() {
    let error_slot = callee_cdecl!(6, u32,);
    (error_slot as *mut u32).write(ERROR_OUT_OF_MEMORY);
}

unsafe fn allocate_heap_block(requested_size: u32) -> u32 {
    if requested_size > MAX_HEAP_REQUEST {
        let _ = callee_cdecl!(5, u32, requested_size);
        set_out_of_memory();
        return 0;
    }

    let mut heap_handle = lf_checker_rt::global::<u32>(HEAP_HANDLE_FILE_VA).read();
    if heap_handle == 0 {
        let _ = callee_cdecl!(2, u32,);
        let _ = callee_cdecl!(3, u32, 30);
        let _ = callee_cdecl!(4, u32, 255);
        heap_handle = lf_checker_rt::global::<u32>(HEAP_HANDLE_FILE_VA).read();
    }

    let allocation_size = if requested_size == 0 { 1 } else { requested_size };
    loop {
        let block = callee_stdcall!(1, u32, heap_handle, 0, allocation_size);
        if block != 0 {
            return block;
        }

        if lf_checker_rt::global::<u32>(HEAP_RETRY_STATE_FILE_VA).read() == 0 {
            set_out_of_memory();
            set_out_of_memory();
            return block;
        }

        if callee_cdecl!(5, u32, requested_size) == 0 {
            set_out_of_memory();
            return block;
        }
    }
}

export!(cdecl, rw_00dfdd67(requested_size: u32) -> u32 {
    unsafe { allocate_heap_block(requested_size) }
});
