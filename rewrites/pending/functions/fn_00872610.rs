// original: 0x00872610 rage::crmtNode::vf1
//! Reset a motion node: run the child-list teardown (callee 1), clear the
//! link words at `this+0xC`/`this+0x10`, and clear `this+8` when it was
//! set. Returns the teardown's answer.
export!(thiscall, rw_00872610(this: *mut u8) -> u32 {
    unsafe {
        let r: u32 = callee_thiscall!(1, u32, this as u32);
        let had_link = *(this.add(8) as *const u32) != 0;
        *(this.add(0x0C) as *mut u32) = 0;
        *(this.add(0x10) as *mut u32) = 0;
        if had_link {
            *(this.add(8) as *mut u32) = 0;
        }
        r
    }
});
