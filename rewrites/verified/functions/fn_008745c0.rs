// original: 0x008745c0 rage::crmtNodeAnimation::vf5
/// Writes the node's debug state to a log: a header line through the node's
/// own describe slot, the parameter block, the attached animation's name and
/// blend weight, and the state flag with its second weight.
export!(thiscall, rw_008745c0(node: u32, log: u32) -> () {
    unsafe {
        let slot = ((node as *const u32).read() + 0x34) as *const u32;
        let describe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot.read() as usize);
        let info = describe(node);
        let mode = ((node + 4) as *const u16).read() as u32;
        let extra = ((info + 4) as *const u32).read();
        callee_cdecl!(2, u32, log, relocated(0x00FC74A8), node, extra, mode);
        let p0 = ((node + 0xC) as *const u32).read();
        let p1 = ((node + 0x10) as *const u32).read();
        callee_cdecl!(3, u32, log, relocated(0x00FC74C4), p0, p1);
        let held = ((node + 0x34) as *const u32).read();
        let anim_name = if held != 0 {
            ((held + 0x1C) as *const u32).read()
        } else {
            relocated(0x00FC7834)
        };
        let weight = ((node + 0x20) as *const f32).read() as f64;
        let bits = weight.to_bits();
        callee_cdecl!(
            4, u32, log, relocated(0x00FC77E8), anim_name,
            (bits & 0xFFFF_FFFF) as u32, (bits >> 32) as u32
        );
        let tag = ((node + 0x1C) as *const u32).read() & 3;
        let flag = if tag != 0 {
            (tag == 3) as u32
        } else if ((node + 0x34) as *const u32).read() != 0 {
            let held2 = ((node + 0x34) as *const u32).read();
            ((((held2 + 6) as *const u8).read() & 1) != 0) as u32
        } else {
            0
        };
        let mode2 = ((node + 0x38) as *const u8).read() as u32;
        let weight2 = ((node + 0x30) as *const f32).read() as f64;
        let bits2 = weight2.to_bits();
        callee_cdecl!(
            5, u32, log, relocated(0x00FC77FC), flag,
            (bits2 & 0xFFFF_FFFF) as u32, (bits2 >> 32) as u32, mode2
        );
    }
});
