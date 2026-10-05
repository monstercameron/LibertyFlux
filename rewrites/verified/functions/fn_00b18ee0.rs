// original: 0x00b18ee0 sweep_flagged_items (proposed)

/// Sweeps the indexed table, retiring items the probe flags 0x15.
///
/// Thiscall with no stack arguments. Runs the gate query (an indirect
/// call through the dispatch slot, thiscall of the gate word) and forms
/// the veto as nonzero gate answer, both mode bytes set, or either
/// veto byte set; on veto it returns the veto byte at once. Otherwise
/// it loads the table for the signed index at +0x2E, asks the counter
/// (thiscall, no stack arguments) how many rows are live and, while the
/// row cursor is below the re-fetched count, fetches each row object
/// (thiscall of the table and the cursor), runs the row's own probe
/// (the vtable slot at +4, thiscall, no stack arguments) and retires
/// rows probing 0x15 (thiscall of this and the row). Returns the last
/// counter answer, or the veto byte on the early path.
lf_checker_rt::export!(thiscall, rw_00b18ee0(this: u32) -> u32 {
    unsafe {
        const GATE_WORD: u32 = 0x017accd8;
        const MODE0: u32 = 0x0105b48f;
        const MODE1: u32 = 0x017ed8d1;
        const VETO0: u32 = 0x01173590;
        const VETO1: u32 = 0x01173591;
        const TABLE_ARRAY: u32 = 0x01295cd8;
        const INDEX_OFF: u32 = 0x2e;
        const RETIRE_TAG: u8 = 0x15;
        let gate = (lf_checker_rt::global::<u32>(GATE_WORD) as *const u32).read_unaligned();
        // The original reaches the gate through its dispatch slot (the
        // contract plants the stub there); the rewrite calls the stub by id.
        let answer: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, gate);
        let mut veto = if answer != 0 {
            1u8
        } else if (lf_checker_rt::global::<u8>(MODE0) as *const u8).read() != 0
            && (lf_checker_rt::global::<u8>(MODE1) as *const u8).read() != 0
        {
            1u8
        } else {
            0u8
        };
        veto |= (lf_checker_rt::global::<u8>(VETO0) as *const u8).read();
        veto |= (lf_checker_rt::global::<u8>(VETO1) as *const u8).read();
        if veto != 0 {
            return veto as u32;
        }
        let index = ((this + INDEX_OFF) as *const i16).read_unaligned() as i32 as u32;
        let table = (lf_checker_rt::relocated(
            TABLE_ARRAY.wrapping_add(index.wrapping_mul(4)),
        ) as *const u32)
            .read_unaligned();
        let mut count: u32 = lf_checker_rt::callee_thiscall!(2, u32, table);
        if (count as i32) <= 0 {
            return count;
        }
        let mut cursor = 0u32;
        loop {
            let row: u32 = lf_checker_rt::callee_thiscall!(3, u32, table, cursor);
            // The original reaches the probe through the row's own table
            // slot, planted with the stub; the rewrite calls it by id.
            let probe: u32 = lf_checker_rt::callee_thiscall!(4, u32, row);
            if (probe as u8) == RETIRE_TAG {
                lf_checker_rt::callee_thiscall!(5, u32, this, row);
            }
            cursor += 1;
            count = lf_checker_rt::callee_thiscall!(2, u32, table);
            if (cursor as i32) >= (count as i32) {
                break;
            }
        }
        count
    }
});
