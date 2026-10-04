// original: 0x00cc7240 registry_teardown
/// Release the two registry lists node by node, drain both queues, release
/// every array entry (and its payload when flagged), then release the array
/// itself and clear its pointer and count. Returns 0.
export!(cdecl, rw_00cc7240() -> u32 {
    unsafe {
        let mut node = *global::<u32>(0x171C0F8);
        while node != 0 {
            let payload = *(node as *const u32);
            if payload != 0 {
                let _: u32 = callee_cdecl!(1, u32, payload);
            }
            node = *(((node) as *const u8).add(4) as *const u32);
        }
        let _: u32 = callee_thiscall!(2, u32, relocated(0x171C0F8));
        node = *global::<u32>(0x171C110);
        while node != 0 {
            let payload = *(node as *const u32);
            if payload != 0 {
                let _: u32 = callee_cdecl!(1, u32, payload);
            }
            node = *(((node) as *const u8).add(4) as *const u32);
        }
        let _: u32 = callee_thiscall!(2, u32, relocated(0x171C110));
        let mut count = *global::<u16>(0x171C108);
        if count > 0 {
            let mut i = 0u32;
            loop {
                let arr = *global::<u32>(0x171C104);
                let entry = *(((arr) as *const u32).add(i as usize));
                if entry != 0 {
                    if *(((entry) as *const u8).add(6) as *const u16) != 0 {
                        let _: u32 = callee_cdecl!(1, u32, *(entry as *const u32));
                    }
                    let _: u32 = callee_cdecl!(1, u32, entry);
                    count = *global::<u16>(0x171C108);
                }
                i += 1;
                if i >= count as u32 {
                    break;
                }
            }
        }
        let _: u32 = callee_cdecl!(1, u32, *global::<u32>(0x171C104));
        *global::<u32>(0x171C104) = 0;
        *global::<u32>(0x171C108) = 0;
        0
    }
});
