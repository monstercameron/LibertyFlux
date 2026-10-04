// original: 0x00cc5590 registry_find_and_detach
/// Find the registered object whose tag word equals `key` in the global
/// registry list and return it (null when absent).
///
/// Each list node holds an object pointer and a next link. When the found
/// object has the attached flag (bit 0x1000 at +0x378) set, the detach helper
/// runs first and the flag is cleared.
export!(cdecl, rw_00cc5590(key: u32) -> u32 {
    unsafe {
        let mut node = *global::<u32>(0x171C0F8);
        while node != 0 {
            let obj = *(node as *const u32);
            if *(obj as *const u32) == key {
                let flags = (obj as *mut u8).add(0x378) as *mut u32;
                if *flags & 0x1000 != 0 {
                    let _: u32 = callee_cdecl!(1, u32, obj);
                    let obj_now = *(node as *const u32);
                    *((obj_now as *mut u8).add(0x378) as *mut u32) &= !0x1000u32;
                    return obj_now;
                }
                return obj;
            }
            node = *(((node) as *const u8).add(4) as *const u32);
        }
        0
    }
});
