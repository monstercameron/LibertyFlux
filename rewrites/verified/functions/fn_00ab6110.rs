// original: 0x00ab6110 idmap_clone (proposed)

/// Deep-copy a wide-node id map into this object.
///
/// Copies the 16-bit bucket count and used count from the source header,
/// allocates a fresh bucket array through the allocator callee, then
/// clones every chain node (key word plus a 34-word body, fresh links, the
/// clone's link cleared) through the node allocator. Returns nothing; the
/// done flag at `+0xB` ends cleared on both the empty and copied paths.
///
/// Callees: 1 = array allocator (cdecl, one word),
/// 2 = node allocator (cdecl, one word).
///
/// Original: 0x00ab6110 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00ab6110(this: u32, src: u32) -> u32 {
    unsafe {
        const ARRAY_ALLOC: u32 = 1;
        const NODE_ALLOC: u32 = 2;
        const TABLE_OFF: u32 = 0;
        const COUNT_OFF: u32 = 4;
        const USED_OFF: u32 = 6;
        const DONE_OFF: u32 = 0x0B;
        const NODE_BYTES: u32 = 0x90;
        const COPY_WORDS: u32 = 0x22;
        const NEXT_OFF: u32 = 0x8C;
        let count = ((src + COUNT_OFF) as *const u16).read_unaligned();
        ((this + COUNT_OFF) as *mut u16).write_unaligned(count);
        ((this + USED_OFF) as *mut u16)
            .write_unaligned(((src + USED_OFF) as *const u16).read_unaligned());
        let (size, overflow) = (count as u32).overflowing_mul(4);
        let bytes = if overflow { 0xFFFF_FFFF } else { size };
        let table = lf_checker_rt::callee_cdecl!(ARRAY_ALLOC, u32, bytes);
        (this as *mut u32).write_unaligned(table);
        let mut i = 0u32;
        while i < count as u32 {
            let src_table = ((src + TABLE_OFF) as *const u32).read_unaligned();
            let mut chain = (((src_table + 4 * i)) as *const u32).read_unaligned();
            let cell = table.wrapping_add(4 * i);
            (cell as *mut u32).write_unaligned(0);
            let mut link = cell;
            while chain != 0 {
                let fresh = lf_checker_rt::callee_cdecl!(NODE_ALLOC, u32, NODE_BYTES);
                if fresh == 0 {
                    (link as *mut u32).write_unaligned(0);
                    chain = ((chain + NEXT_OFF) as *const u32).read_unaligned();
                    link = NEXT_OFF; // original links the next clone at 0x8c past null
                    if chain == 0 {
                        break;
                    }
                    continue;
                }
                (fresh as *mut u32)
                    .write_unaligned((chain as *const u32).read_unaligned());
                let mut w = 0u32;
                while w < COPY_WORDS {
                    let v = (((chain + 4) + 4 * w) as *const u32).read_unaligned();
                    (((fresh + 4) + 4 * w) as *mut u32).write_unaligned(v);
                    w += 1;
                }
                ((fresh + NEXT_OFF) as *mut u32).write_unaligned(0);
                (link as *mut u32).write_unaligned(fresh);
                chain = ((chain + NEXT_OFF) as *const u32).read_unaligned();
                link = fresh.wrapping_add(NEXT_OFF);
            }
            i += 1;
        }
        ((this + DONE_OFF) as *mut u8).write(0);
        0
    }
});
