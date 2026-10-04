// original: 0x009e2bd0 CPed::vf0
/// 0x009E2BD0 (CPed::vf0, deleting-destructor shape): run the destructor,
/// then release through the pool helper found via a global when flags bit 0
/// is set. Returns the object pointer. (thiscall/1)
export!(thiscall, rw_009e2bd0(this: *mut u8, flags: u32) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        if flags & 1 != 0 {
            let pool = *global::<u32>(0x18B6F1C);
            callee_thiscall!(2, u32, pool, this as u32);
        }
        this as u32
    }
});
