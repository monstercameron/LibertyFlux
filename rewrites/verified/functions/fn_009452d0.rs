// original: 0x009452d0 streaming_list_find (proposed)

/// Find the streaming list entry selected by the key byte.
///
/// Key 3 returns the primary head global, key 4 the secondary head global.
/// Any other key walks the lists from `this + 0x17d0` via the next link at
/// `+0x18`: each node holds an item count byte at `+0x13` and a pointer
/// array at `+0x14`, scanned for the first live entry whose tag byte at
/// `+0xa` equals the key. Returns the entry, or 0.
///
/// Original: 0x009452d0 (thiscall, one stack argument; callee pops 4).
lf_checker_rt::export!(thiscall, rw_009452d0(this: u32, key: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x17D0;
        const COUNT: u32 = 0x13;
        const ITEMS: u32 = 0x14;
        const NEXT: u32 = 0x18;
        const TAG: u32 = 0xA;
        const PRIMARY: u32 = 0x011D7644;
        const SECONDARY: u32 = 0x011D7640;
        let want = key as u8;
        if want == 3 {
            return lf_checker_rt::global::<u32>(PRIMARY).read();
        }
        if want == 4 {
            return lf_checker_rt::global::<u32>(SECONDARY).read();
        }
        let mut node = ((this + HEAD) as *const u32).read_unaligned();
        loop {
            let count = ((node + COUNT) as *const u8).read() as u32;
            let items = ((node + ITEMS) as *const u32).read_unaligned();
            let mut i = 0u32;
            while i < count {
                let entry =
                    ((items + i * 4) as *const u32).read_unaligned();
                if entry != 0 && ((entry + TAG) as *const u8).read() == want {
                    return entry;
                }
                i += 1;
            }
            node = ((node + NEXT) as *const u32).read_unaligned();
            if node == 0 {
                return 0;
            }
        }
    }
});
