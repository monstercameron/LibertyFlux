// original: 0x00cc7110 items_notify_matching_tags
/// Walk the item iterator; for each item whose tag (`+0xC`) appears in the
/// global tag list, invoke the notify helper. Returns 0.
export!(stdcall, rw_00cc7110(obj: u32) -> u32 {
    unsafe {
        let mut item: u32 = callee_thiscall!(1, u32, obj, 0, 2);
        while item != 0 {
            let list = global::<u32>(0x1051598);
            if *list != 0xFFFF_FFFF {
                let want = *(((item) as *const u8).add(0xC) as *const u32);
                let mut i = 0usize;
                loop {
                    if *list.add(i) == want {
                        let _: u32 = callee_thiscall!(2, u32, obj, item);
                    }
                    i += 1;
                    if *list.add(i) == 0xFFFF_FFFF {
                        break;
                    }
                }
            }
            item = callee_thiscall!(3, u32, obj, 0, 2);
        }
        0
    }
});
