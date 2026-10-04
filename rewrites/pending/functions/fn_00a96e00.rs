// original: 0x00A96E00 assemble_node_record
/// Assemble a node record from fifteen source pointers (original 0x00A96E00).
///
/// Fetches a fresh node from the node pool; when the pool is empty it marks
/// the owner object instead and returns null. Otherwise it copies one to four
/// words out of each source pointer into fixed slots of the node, raises the
/// node's ready flag, clears its sequence word, and hands the node to the
/// owner's sink, returning whatever the sink answers.
export!(thiscall, rw_a96e00(
    this: u32,
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32,
    a5: u32, a6: u32, a7: u32, a8: u32, a9: u32,
    a10: u32, a11: u32, a12: u32, a13: u32, a14: u32,
) -> u32 {
    const POOL: u32 = 0x0130_5D30;
    const READY_FLAG: u8 = 1;
    let node = callee_thiscall!(1, u32, relocated(POOL));
    if node == 0 {
        unsafe { (this as *mut u8).add(0x15).write(READY_FLAG) };
        return 0;
    }
    unsafe {
        let dst = node as *mut u32;
        let srcs: [u32; 15] = [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14];
        // Words copied per source; slots are packed front to back except for
        // a three-word gap after the eleventh source's word.
        let widths: [usize; 15] = [4, 4, 4, 4, 4, 4, 2, 2, 2, 1, 1, 1, 4, 4, 4];
        let mut slot: usize = 0x10 / 4;
        for (i, &src) in srcs.iter().enumerate() {
            if i == 12 {
                slot += 3; // gap at node+0x94..0x9c, left untouched
            }
            let from = src as *const u32;
            for w in 0..widths[i] {
                dst.add(slot + w).write(from.add(w).read());
            }
            slot += widths[i];
        }
        (node as *mut u8).add(0xf0).write(READY_FLAG);
        dst.add(0xf4 / 4).write(0);
        callee_thiscall!(2, u32, this.wrapping_add(8), node)
    }
});
