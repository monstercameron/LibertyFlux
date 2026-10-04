// original: 0x00952430 slot_table_sweep_teardown
/// Sweeps the slot table tearing down every entry bound to the key.
///
/// Keys of 1 or less return at once. Otherwise all 0x818 slots are scanned:
/// a live entry whose binding word matches the key is released by its type
/// nibble (virtual pair for types 2 and 4; type 3 first consults a virtual
/// precheck that can skip the release, then unlinks a live side binding;
/// type 1 goes through the registry release) and the slot is cleared with a
/// 0xffff tag. Returns the key.
export!(cdecl, rw_00952430(key: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0x818;
        const TABLE: u32 = 0x0120_8970;
        const ENTRY_STRIDE: u32 = 8;
        const BIND_OFF: u32 = 0x68;
        const TYPE_OFF: u32 = 0x28;
        const VT_PRE: u32 = 0x98;
        const VT_REL: u32 = 0x30;
        const VT_CHECK: u32 = 0x128;
        const LINK_OFF: u32 = 0xB30;
        const FLAG_OFF: u32 = 0x26C;
        const FLAG_KEEP: u32 = 0xFFFF_FFFB;
        const FREED_TAG: u16 = 0xFFFF;
        if (key as i32) <= 1 {
            return key;
        }
        let mut entry = relocated(TABLE);
        let mut left = COUNT;
        while left > 0 {
            let obj = *(entry as *const u32);
            if obj != 0 && *(obj.wrapping_add(BIND_OFF) as *const u32) == key {
                let typ = (*((obj.wrapping_add(TYPE_OFF)) as *const u32) >> 6) & 0xF;
                if typ == 2 || typ == 4 {
                    let vt = *(obj as *const u32);
                    let pre: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(
                            *((vt.wrapping_add(VT_PRE)) as *const u32) as usize,
                        );
                    pre(obj);
                    let vt = *(obj as *const u32);
                    let rel: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(
                            *((vt.wrapping_add(VT_REL)) as *const u32) as usize,
                        );
                    rel(obj, 0);
                } else if typ == 3 {
                    let vt = *(obj as *const u32);
                    let check: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(
                            *((vt.wrapping_add(VT_CHECK)) as *const u32) as usize,
                        );
                    if check(obj) & 0xFF == 0 {
                        let vt = *(obj as *const u32);
                        let pre: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(
                                *((vt.wrapping_add(VT_PRE)) as *const u32) as usize,
                            );
                        pre(obj);
                        let vt = *(obj as *const u32);
                        let rel: extern "thiscall" fn(u32, u32) -> u32 =
                            core::mem::transmute(
                                *((vt.wrapping_add(VT_REL)) as *const u32) as usize,
                            );
                        rel(obj, 0);
                        let link_addr = obj.wrapping_add(LINK_OFF);
                        let link = *(link_addr as *const u32);
                        if link != 0 {
                            callee_thiscall!(4, u32, link, link_addr);
                            let flags = (obj.wrapping_add(FLAG_OFF)) as *mut u32;
                            *flags &= FLAG_KEEP;
                            *(link_addr as *mut u32) = 0;
                        }
                    }
                } else if typ == 1 {
                    callee_cdecl!(5, u32, obj);
                }
                *(entry as *mut u32) = 0;
                *((entry.wrapping_add(4)) as *mut u16) = FREED_TAG;
            }
            entry = entry.wrapping_add(ENTRY_STRIDE);
            left -= 1;
        }
        key
    }
});
