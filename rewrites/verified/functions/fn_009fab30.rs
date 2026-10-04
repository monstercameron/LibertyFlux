// original: 0x009fab30 playstat_chain_report_emit
/// Build and emit a report for the object behind the global slot chain.
///
/// Constructs a temporary report object on the stack, tags it, attaches
/// the calling object's head word, then follows the global slot chain to
/// find the target: an empty chain stores zero, otherwise the resolved
/// target is queried and its answer stored. Emits the report and tears
/// the object down. Returns nothing meaningful.
export!(cdecl, rw_009fab30(obj: u32, extra: u32) -> u32 {
    unsafe {
        // File VA of the report tag object (relocated; see 5D0).
        const TAG: u32 = 0x00e998a8;
        let mut frame = [0u32; 32];
        let temp = frame.as_mut_ptr();
        let aux = frame.as_mut_ptr().add(13);
        callee_thiscall!(1, u32, temp as u32, 3, 0x28);
        temp.write(relocated(TAG));
        let head = (obj as *const u32).read();
        let token = callee_cdecl!(2, u32, head, 0);
        callee_thiscall!(3, u32, temp as u32, token, head);
        callee_cdecl!(4, u32, extra, aux as u32);
        let slot = global::<u32>(0x01036f14).read();
        let stored = if slot == 0xFFFFFFFF {
            0
        } else {
            let entry = global::<u32>(0x011a8808).add(slot as usize).read();
            if entry == 0 {
                0
            } else {
                let inner = (entry as *const u32).add(0x598 / 4).read();
                let target = (inner as *const u32).add(0x228 / 4).read();
                let query = if target == 0 { 0 } else { target.wrapping_add(0x70) };
                callee_thiscall!(5, u32, query)
            }
        };
        frame[14] = stored;
        callee_cdecl!(6, u32, temp as u32, 0x3c);
        callee_thiscall!(7, u32, temp as u32);
        // Stack-cookie check: argument excluded from comparison (see 740).
        callee_thiscall!(8, u32, 0);
        0
    }
});
