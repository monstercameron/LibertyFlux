// original: 0x00872640 rage::crmtNode::vf5
//! Report a motion node through the trace sink: resolve the node kind via
//! slot 0x34 of its table (callee 1), then emit the node record (callee 2)
//! and the parent record (callee 3), both carrying the caller's tag `a0`.
//! Returns the parent record's answer.
/// File VAs of the trace format strings used by the node reporters.
const TRACE_NODE_FMT: u32 = 0x00FC74A8;
const TRACE_PARENT_FMT: u32 = 0x00FC74C4;

export!(thiscall, rw_00872640(this: *mut u8, a0: u32) -> u32 {
    unsafe {
        let vt = *(this as *const u32);
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(0x34)) as *const u32) as usize);
        let info = kind_of(this as u32);
        let refs = *(this.add(4) as *const u16) as u32;
        let type_word = *((info.wrapping_add(4)) as *const u32);
        callee_cdecl!(2, u32, a0, relocated(TRACE_NODE_FMT), this as u32, type_word, refs);
        let c = *(this.add(0x0C) as *const u32);
        let d = *(this.add(0x10) as *const u32);
        callee_cdecl!(3, u32, a0, relocated(TRACE_PARENT_FMT), c, d)
    }
});
