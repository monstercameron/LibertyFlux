// original: 0x00c6d3d0 anim_dict_create (proposed)

/// Allocate a dictionary object and publish it in its global.
///
/// Allocates the fixed-size header; on failure the global is cleared
/// and null returned. Otherwise the object is initialised by the shared
/// constructor with (capacity, slots, word size 16), stored in the
/// global, and returned.
///
/// Original: cdecl with two stack words, two calls, writes one global.
lf_checker_rt::export!(cdecl, rw_00c6d3d0(capacity: u32, slots: u32) -> u32 {
    unsafe {
        const DICT: u32 = 0x016D_D5D0;
        const HEADER_BYTES: u32 = 0x1C;
        const WORD_SIZE: u32 = 0x10;
        const ALLOC: u32 = 1;
        const CTOR: u32 = 2;

        let mem: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, HEADER_BYTES);
        if mem == 0 {
            unsafe {
                (lf_checker_rt::relocated(DICT) as *mut u32).write_unaligned(0)
            };
            return 0;
        }
        let obj: u32 =
            lf_checker_rt::callee_thiscall!(CTOR, u32, mem, capacity, slots, WORD_SIZE);
        unsafe { (lf_checker_rt::relocated(DICT) as *mut u32).write_unaligned(obj) };
        obj
    }
});
