// original: 0x00a64480 intel_table_poll
/// Polls the first live entry of the five-slot table at +0x44.
///
/// Calls the entry's handler slot (vtable +0x34) with the context field at
/// +0x40 and the given float bits. When the handler reports false, retries
/// with the first enabled entry of the three-slot table at +0x70 (an entry
/// is enabled when it is non-null and its flag byte at +0xC has bit 0
/// clear). Returns the handler's answer. When no entry is available the
/// original dereferences a null table pointer and faults; this rewrite
/// faults identically.
export!(thiscall, rw_00a64480(this: u32, farg: u32) -> u32 {
    unsafe {
        type Handler = extern "thiscall" fn(u32, u32, u32) -> u32;
        unsafe fn call_handler(obj: u32, field: u32, farg: u32) -> u32 {
            let vtable = *(obj as *const u32);
            let slot = *((vtable + 0x34) as *const u32);
            let handler: Handler = core::mem::transmute(slot as usize);
            handler(obj, field, farg)
        }
        let field = *((this + 0x40) as *const u32);
        let mut obj = 0u32;
        let mut i = 0u32;
        while i < 5 {
            let cand = *((this + 0x44 + i.wrapping_mul(4)) as *const u32);
            if cand != 0 {
                obj = cand;
                break;
            }
            i += 1;
        }
        let first = call_handler(obj, field, farg);
        if (first as u8) != 0 {
            return first;
        }
        let mut obj2 = 0u32;
        let mut j = 0u32;
        while j < 3 {
            let cand = *((this + 0x70 + j.wrapping_mul(4)) as *const u32);
            if cand != 0 && *((cand + 0xC) as *const u8) & 1 == 0 {
                obj2 = cand;
                break;
            }
            j += 1;
        }
        call_handler(obj2, field, farg)
    }
});
