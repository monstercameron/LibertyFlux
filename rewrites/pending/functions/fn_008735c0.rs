// original: 0x008735c0 rage::crmtRequestInsert::vf2
use lf_checker_rt::{callee_thiscall, export};

/// `rage::crmtRequestInsert::vf2`: build a node and insert or release it.
///
/// Builds through the factory helper on the inner object at +0x14 (zero
/// when there is none), tests the probe object through its vtable, then
/// either inserts the node into the list at +0x18 or releases it.
/// Returns the built node (zero only when there is no inner object).
export!(thiscall, rw_008735c0(this_: u32, a0: u32, probe: u32) -> u32 {
    unsafe {
        let inner = *((this_.wrapping_add(0x14)) as *const u32);
        if inner == 0 {
            return 0;
        }
        let node = callee_thiscall!(1, u32, inner, a0, 0);
        if probe == 0 {
            return node;
        }
        let vt = *(probe as *const u32);
        let test: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(*((vt.wrapping_add(0x20)) as *const u32));
        if (test(probe) as u8) != 0 {
            let list = *((this_.wrapping_add(0x18)) as *const u32);
            callee_thiscall!(3, u32, probe, list, node);
        } else {
            callee_thiscall!(4, u32, probe, node, 1);
        }
        node
    }
});
