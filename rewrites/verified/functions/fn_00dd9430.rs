// original: 0x00dd9430 UIMontageClip::vf103
/// `UIMontageClip::vf103`: resolve two handles and dispatch on their tags.
///
/// Asks the engine object for one handle derived from the argument and one
/// derived from a global table word, checks the first handle's slot-0 answer
/// against a helper, then compares the second handle's slot `0x4c` tag with
/// the `field_300` member's tag. Equal tags take one member slot, differing
/// tags another; a first-stage mismatch skips both. Always finishes through
/// the object's own slot `0x1a0` and returns its answer.
export!(thiscall, rw_00dd9430(this_ptr: u32, arg0: u32) -> u32 {
    unsafe {
        let engine = relocated(0x1981a4c);
        let ebp = callee_thiscall!(3, u32, engine, arg0);
        let table = global::<u32>(0x18b6c8c).read();
        let word = ((table + 0x200) as *const u32).read();
        let ebx = callee_thiscall!(3, u32, engine, word);
        let probe_slot = ((ebx as *const u32).read() as *const u32).read();
        let probe_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(probe_slot as usize);
        let probe = probe_of(ebx);
        if callee_cdecl!(4, u32, relocated(0xefb674)) != probe {
            let fin_slot =
                ((((this_ptr as *const u32).read() + 0x1a0)) as *const u32).read();
            let fin: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(fin_slot as usize);
            return fin(this_ptr);
        }
        let member = ((this_ptr as *const u32).add(0x300 / 4)).read();
        let left_slot = (((member as *const u32).read() + 0x4c) as *const u32).read();
        let left_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(left_slot as usize);
        let left = left_of(member);
        let tag_slot = (((ebp as *const u32).read() + 0x4c) as *const u32).read();
        let tag_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(tag_slot as usize);
        if tag_of(ebp) != left {
            let v = callee_thiscall!(5, u32, ebx);
            callee_thiscall!(7, u32, this_ptr, v);
        } else {
            let v = callee_thiscall!(5, u32, ebx);
            callee_thiscall!(6, u32, this_ptr, v);
        }
        let fin_slot =
            ((((this_ptr as *const u32).read() + 0x1a0)) as *const u32).read();
        let fin: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(fin_slot as usize);
        fin(this_ptr)
    }
});
