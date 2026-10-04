// original: 0x00663f70 report_id_match
/// Migration id check: verify the looked-up pair, report the verdict.
///
/// thiscall/2, returns void. Looks a candidate id pair up through callee 1;
/// when the lookup succeeds and the pair matches the record argument, reports
/// it as confirmed, otherwise reports the fallback argument, both through
/// callee 2 with the task's result word.
export!(thiscall, rw_00663f70(this_ptr: u32, record: u32, fallback: u32) -> u32 {
    unsafe {
        let mut pair = [0u32, 0u32];
        let ok: u32 = callee_thiscall!(1, u32, pair.as_mut_ptr() as u32);
        let confirmed = (ok as u8) != 0
            && (pair[0] != u32::MAX || pair[1] != u32::MAX)
            && ((record + 0x38) as *const u32).read() == pair[0]
            && ((record + 0x3c) as *const u32).read() == pair[1];
        let session = ((this_ptr + 0x60) as *const u32).read();
        if confirmed {
            callee_thiscall!(2, u32, session.wrapping_add(0x48), record, 0,
                this_ptr.wrapping_add(0x94));
        } else {
            callee_thiscall!(2, u32, session.wrapping_add(0x48), record, fallback,
                this_ptr.wrapping_add(0x94));
        }
        0
    }
});
