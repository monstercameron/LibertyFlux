// original: 0x00AF7060 veh_node_collect_bitmask (proposed)

/// Collect a bitmask over a node chain from two alternative probers.
///
/// Walks the node chain whose head is at `list + 0x1C`, following `+0x18`
/// links and skipping nodes whose flag byte at `+0x2C` is clear. Each live
/// node is classified twice by callee 1 with different tags: a zero first
/// verdict runs prober A (callee 2 decodes the node value into a scratch
/// buffer, callee 3 reduces the buffer to a bit index, and that bit is set
/// in `out + 0x20`); otherwise a zero second verdict runs prober B (callee 2
/// decodes, callee 4 reduces to a full replacement word for `out + 0x20`).
/// The node value is `node + 0x20` when `node + 0x24` is nonzero, else zero.
/// Scratch buffers live in a local array (the original passes its own frame
/// addresses; the contract skips those pointer arguments and compares the
/// pointed-to words instead). Callee 5 is the stack-cookie check.
///
/// Original: 0x00AF7060 (cdecl, two stack arguments).
lf_checker_rt::export!(cdecl, rw_00AF7060(list: u32, out: u32) -> u32 {
    unsafe {
        const CLASSIFY: u32 = 1;
        const SECOND_CLASSIFY: u32 = 2;
        const DECODE: u32 = 3;
        const REDUCE_BIT: u32 = 4;
        const REDUCE_WORD: u32 = 5;
        const COOKIE_CHECK: u32 = 6;
        const TAG_A: u32 = 0xEA8AE8;
        const TAG_A_DEC: u32 = 0xEA8AF0;
        const TAG_B: u32 = 0xEA8AF4;
        const TAG_B_DEC: u32 = 0xEA8AFC;
        const HEAD: u32 = 0x1C;
        const NEXT: u32 = 0x18;
        const FLAG: u32 = 0x2C;
        const VALUE: u32 = 0x20;
        const PRESENT: u32 = 0x24;
        const OUT_MASK: u32 = 0x20;
        let mut scratch = [0u32; 4];
        let buf = scratch.as_mut_ptr() as u32;
        let mut node = ((list + HEAD) as *const u32).read_unaligned();
        while node != 0 {
            if ((node + FLAG) as *const u8).read() != 0 {
                let head = (node as *const u32).read_unaligned();
                if lf_checker_rt::callee_cdecl!(CLASSIFY, u32, head, lf_checker_rt::relocated(TAG_A)) == 0 {
                    let v = if ((node + PRESENT) as *const u32).read_unaligned() == 0 {
                        0
                    } else {
                        ((node + VALUE) as *const u32).read_unaligned()
                    };
                    lf_checker_rt::callee_cdecl!(DECODE, u32, v, lf_checker_rt::relocated(TAG_A_DEC), buf);
                    let bit = lf_checker_rt::callee_cdecl!(REDUCE_BIT, u32, buf);
                    let slot = (out + OUT_MASK) as *mut u32;
                    slot.write_unaligned(slot.read_unaligned() | (1u32 << (bit & 31)));
                } else if lf_checker_rt::callee_cdecl!(SECOND_CLASSIFY, u32, head, lf_checker_rt::relocated(TAG_B)) == 0 {
                    let v = if ((node + PRESENT) as *const u32).read_unaligned() == 0 {
                        0
                    } else {
                        ((node + VALUE) as *const u32).read_unaligned()
                    };
                    lf_checker_rt::callee_cdecl!(DECODE, u32, v, lf_checker_rt::relocated(TAG_B_DEC), buf);
                    let word = lf_checker_rt::callee_cdecl!(REDUCE_WORD, u32, buf, 0u32, 0x10u32);
                    ((out + OUT_MASK) as *mut u32).write_unaligned(word);
                }
            }
            node = ((node + NEXT) as *const u32).read_unaligned();
        }
        lf_checker_rt::callee_cdecl!(COOKIE_CHECK, u32,);
        0
    }
});
