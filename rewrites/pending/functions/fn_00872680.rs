// original: 0x00872680 rage::crmtNode::vf6
//! Check a motion node pointer against its tree: report through the trace
//! sink (callee 1) when the header word mismatches, then pick the indexed
//! child (or null for a non-positive index) and report again when it
//! differs from `this+0xC`. Returns the last report's answer, or `this+0xC`
//! when the second check passes quietly.
/// File VAs of the trace format strings used by the node checker.
const TRACE_BAD_TREE_FMT: u32 = 0x00FC7508;
const TRACE_BAD_PARENT_FMT: u32 = 0x00FC7524;

export!(thiscall, rw_00872680(this: *mut u8, a0: *const u8) -> u32 {
    unsafe {
        let have = *(this.add(8) as *const u32);
        if *(a0.add(4) as *const u32) != have {
            callee_cdecl!(1, u32, a0 as u32, relocated(TRACE_BAD_TREE_FMT), have);
        }
        let idx = *(a0.add(0x108) as *const i32);
        let picked = if idx <= 0 {
            0
        } else {
            *((a0 as u32)
                .wrapping_add((idx as u32).wrapping_mul(4))
                .wrapping_add(4) as *const u32)
        };
        let want = *(this.add(0x0C) as *const u32);
        if picked != want {
            callee_cdecl!(1, u32, a0 as u32, relocated(TRACE_BAD_PARENT_FMT), want)
        } else {
            want
        }
    }
});
