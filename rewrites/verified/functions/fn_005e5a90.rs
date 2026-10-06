// original: 0x005e5a90 cached_factory_init
/// Lazily create and return the shared factory object.
///
/// Returns the cached object when one is already stored. Otherwise allocates
/// a 0x24-byte block through the current thread's allocator (reached via TLS
/// slot 0), initializes it with the factory setup step, caches the result and
/// returns it. A failed allocation stores and returns null.
export!(cdecl, rw_005E5A90() -> u32 {
    unsafe {
        let cached = *global::<u32>(FACTORY_CACHE);
        if cached != 0 {
            return cached;
        }
        let tls0 = tls_slot(0);
        let inner = *((tls0 + 8) as *const u32);
        let vt = *(inner as *const u32);
        let tgt = *((vt as *const u8).add(8) as *const u32);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(tgt as usize);
        let obj = alloc(inner, FACTORY_SIZE, FACTORY_ALIGN, 0);
        if obj == 0 {
            *global::<u32>(FACTORY_CACHE) = 0;
            return 0;
        }
        let init = callee_thiscall!(2, u32, obj);
        *global::<u32>(FACTORY_CACHE) = init;
        init
    }
});
