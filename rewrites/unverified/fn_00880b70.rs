// original: 0x00880B70 audio_mixer_buffer_create

/// Allocate a fixed-size audio buffer and initialize it with the caller's
/// source pointer.
///
/// The cdecl allocator receives a 0x47c-byte request and 16-byte alignment. A
/// null allocation returns zero without calling the initializer. Otherwise,
/// the initializer receives the allocated buffer in ECX, followed by the
/// original source pointer and a 0x200-byte length; its EAX result is returned.
/// The wrapper uses stdcall cleanup for its one stack argument.
lf_checker_rt::export!(stdcall, rw_00880b70(source: u32) -> u32 {
    unsafe {
        const BUFFER_SIZE_BYTES: u32 = 0x47C;
        const BUFFER_ALIGNMENT: u32 = 0x10;
        const INITIAL_LENGTH_BYTES: u32 = 0x200;
        const ALLOCATE_CALLEE: u32 = 1;
        const INITIALIZE_CALLEE: u32 = 2;

        let buffer = lf_checker_rt::callee_cdecl!(
            ALLOCATE_CALLEE, u32, BUFFER_SIZE_BYTES, BUFFER_ALIGNMENT
        );
        if buffer == 0 {
            return 0;
        }

        lf_checker_rt::callee_thiscall!(
            INITIALIZE_CALLEE,
            u32,
            buffer,
            source,
            INITIAL_LENGTH_BYTES
        )
    }
});
