// original: 0x0089F710 aud_poll_hooks
// ---------------------------------------------------------------------------
// 0x0089F710: poll the 16 registered audio hooks. Fills a scratch table
// through the enumerator, then tests each non-null entry until one rejects.
// Returns 1 when every entry accepts, else 0.
// ---------------------------------------------------------------------------
export!(cdecl, rw_0089F710() -> u32 {
    let mut table = [0u32; 16];
    callee_stdcall!(1, u32, 0xF, table.as_mut_ptr() as u32, 0x10, 1);
    for i in 0..16 {
        let hook = table[i];
        if hook != 0 {
            let accepted = callee_thiscall!(2, u32, hook);
            if accepted & 0xFF == 0 {
                return 0;
            }
        }
    }
    1
});
