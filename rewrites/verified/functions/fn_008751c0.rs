// original: 0x008751c0 source32_combine
/// Link live motion sources into a target list.
///
/// Scans the thirty-two source handles at offsets `0x14..` of the object.
/// For each non-null handle, builds a node through the allocator helper
/// (stubbed), bumps its count word, runs the node init helper (stubbed),
/// appends the node to the tail of the target's `0x1C` chain (or starts
/// the chain), and back-links the node to the target. When the flag
/// argument is non-zero, runs the finish helper (stubbed) on the target.
export!(thiscall, rw_008751c0(this: u32, tag: u32, finish: u32, head: u32) -> u32 {
    unsafe {
        const FIRST_WORD: usize = 0x14 / 4;
        const COUNT: usize = 32;
        const CHAIN_NEXT: usize = 0x10 / 4;
        const HEAD_LIST: usize = 0x1C / 4;
        const NODE_BACK: usize = 0xC / 4;
        let src = (this as *const u32).add(FIRST_WORD);
        let mut i: usize = 0;
        while i < COUNT {
            let handle = src.add(i).read();
            if handle != 0 {
                let node: u32 = callee_thiscall!(1, u32, handle, tag, 0);
                let count = (node as *mut u16).add(2);
                count.write(count.read().wrapping_add(1));
                let _: u32 = callee_thiscall!(2, u32, node);
                let first = (head as *const u32).add(HEAD_LIST).read();
                if first == 0 {
                    (head as *mut u32).add(HEAD_LIST).write(node);
                } else {
                    let mut link = first;
                    loop {
                        let next = (link as *const u32).add(CHAIN_NEXT).read();
                        if next == 0 {
                            (link as *mut u32).add(CHAIN_NEXT).write(node);
                            break;
                        }
                        link = next;
                    }
                }
                (node as *mut u32).add(NODE_BACK).write(head);
            }
            i += 1;
        }
        if finish != 0 {
            let _: u32 = callee_stdcall!(3, u32, head, 1);
        }
        0
    }
});
