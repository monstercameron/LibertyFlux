// original: 0x00da8af0 remap_task_state
/// Remap a task state word through the gated state table.
///
/// Looks the handle up through two chained calls and requires two flag bits
/// in the resulting record; then advances the first state word from 1 to 2
/// and remaps the second through a four-entry table (0x6A->0x6B, 0x70->0x71,
/// 0x77->0x79, 0x78->0x7A, anything else unchanged). The original dispatches
/// through a jump table; the rewrite states the mapping directly.
export!(cdecl, rw_00da8af0(a: u32, b: u32, c: u32) -> () {
    unsafe {
        /// Handle offset adjusted before the first lookup.
        const HANDLE_ADJ: u32 = 0x2b0;
        /// Record field offsets.
        const NEXT: u32 = 0x18;
        const FLAGS: u32 = 0x20;
        let h: u32 = callee_thiscall!(1, u32, a.wrapping_add(HANDLE_ADJ));
        if h == 0 {
            return;
        }
        let rec: u32 = callee_cdecl!(2, u32, ((h.wrapping_add(NEXT)) as *const u32).read_unaligned());
        let w = ((rec.wrapping_add(FLAGS)) as *const u32).read_unaligned();
        if w >> 5 & 1 == 0 {
            return;
        }
        if w >> 12 & 1 == 0 {
            return;
        }
        if ((b) as *const u32).read_unaligned() == 1 {
            ((b) as *mut u32).write_unaligned(2);
        }
        let nv = match ((c) as *const u32).read_unaligned() {
            0x6a => 0x6b,
            0x70 => 0x71,
            0x77 => 0x79,
            0x78 => 0x7a,
            _ => return,
        };
        ((c) as *mut u32).write_unaligned(nv);
    }
});
