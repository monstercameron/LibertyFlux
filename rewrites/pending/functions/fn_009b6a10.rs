// original: 0x009b6a10 NativeImpl_DOES_CAM_EXIST
/// Look a handle up in the global input pool and report whether it is usable.
///
/// Resolves `handle` through the pool at the global slot (thiscall/1,
/// stubbed): a null answer means absent. A nonzero `force` accepts any
/// resolved object, otherwise the object is rejected while status bit 1 at
/// +0x13c is set.
export!(stdcall, rw_009b6a10(handle: u32, force: u8) -> u8 {
    unsafe {
        const POOL: u32 = 0x012FB1A0;
        const STATUS_OFF: u32 = 0x13C;
        const BUSY_BIT: u8 = 2;
        let pool = *global::<u32>(POOL);
        let obj = callee_thiscall!(1, u32, pool, handle);
        if obj == 0 {
            return 0;
        }
        if force != 0 {
            return 1;
        }
        if *((obj + STATUS_OFF) as *const u8) & BUSY_BIT != 0 {
            return 0;
        }
        1
    }
});