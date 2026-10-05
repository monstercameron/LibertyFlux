// original: 0x00AD1A90 audio_get_or_create_handle (proposed)

/// Return the cached audio handle, creating it on first use.
///
/// If the handle global is non-zero it is returned as is. Otherwise a block
/// is allocated (cdecl/2: arena, 0), resolved through the handle table
/// (the table helper reads only its second pushed word, verified from its
/// code, so the ignored first word travels in ecx), wrapped into an object
/// (thiscall/0), stored into the handle global and returned. Takes no
/// arguments (cdecl/0).
lf_checker_rt::export!(cdecl, rw_00ad1a90() -> u32 {
    unsafe {
        const ALLOC: u32 = 1;
        const RESOLVE: u32 = 2;
        const WRAP: u32 = 3;
        const HANDLE: u32 = 0x0154E18C;
        const ARENA: u32 = 0x00EA6340;
        const TABLE_KEY: u32 = 0x0103EED4;
        let cached = lf_checker_rt::global::<u32>(HANDLE).read();
        if cached != 0 {
            return cached;
        }
        let blk = lf_checker_rt::callee_cdecl!(ALLOC, u32, lf_checker_rt::relocated(ARENA), 0u32);
        let key = lf_checker_rt::global::<u32>(TABLE_KEY).read();
        let resolved = lf_checker_rt::callee_thiscall!(RESOLVE, u32, blk, key);
        let obj = lf_checker_rt::callee_thiscall!(WRAP, u32, resolved);
        lf_checker_rt::global::<u32>(HANDLE).write(obj);
        obj
    }
});
