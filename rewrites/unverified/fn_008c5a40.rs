// original: 0x008C5A40 Text_ResolveLabel
/// Hash the label and test the owner, then search its active descriptor
/// records. The out flag is passed by pointer to each intercepted resolver;
/// AL is the Boolean result and the upper EAX bytes retain the last helper.
lf_checker_rt::export!(thiscall, rw_008c5a40(this: u32, label: u32) -> u32 {
    const HASH_HELPER: u32 = 1;
    const INITIAL_RESOLVER: u32 = 2;
    const NULL_LABEL_HANDLER: u32 = 3;
    const RECORD_RESOLVER: u32 = 4;
    const DESCRIPTOR_COUNT: u32 = 14;
    unsafe {
        if label == 0 {
            let result = lf_checker_rt::callee_thiscall!(NULL_LABEL_HANDLER, u32, this);
            return result & 0xFFFF_FF00;
        }
        let hash = lf_checker_rt::callee_cdecl!(HASH_HELPER, u32, label, 0);
        let mut found = 0u32;
        let found_ptr = &mut found as *mut u32 as u32;
        let mut result = lf_checker_rt::callee_thiscall!(INITIAL_RESOLVER, u32,
                                                         this, hash, found_ptr);
        if found != 0 { return (result & 0xFFFF_FF00) | 1; }
        if (this.wrapping_add(0x261) as *const u8).read() != 0 {
            for index in 0..DESCRIPTOR_COUNT {
                if (this.wrapping_add(0xF0 + index) as *const u8).read() != 0 {
                    found = 0;
                    let record = this.wrapping_add(0x10 + index * 8);
                    result = lf_checker_rt::callee_thiscall!(RECORD_RESOLVER, u32,
                                                              record, hash, found_ptr);
                    if found != 0 { return (result & 0xFFFF_FF00) | 1; }
                }
            }
        }
        result & 0xFFFF_FF00
    }
});
