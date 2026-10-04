// original: 0x005e7390 WANTED_STARS_ARE_FLASHING
use lf_k2_rt::{callee_addr, export, global};
/// Report whether the wanted stars are flashing: ask the engine about
/// the currently indexed object (the engine takes the object pointer in
/// ECX and no stack arguments). When the engine's low answer byte is
/// zero the result is 0; otherwise the result is whether the object's
/// flag dword is nonzero. The answer's low byte is stored
/// (zero-extended) in the return slot.
export!(cdecl, rw_005e7390(ctx: *const u32) -> u32 {
    unsafe {
        let table = global::<u32>(0x118E7F8);
        let obj = *table.add(*global::<u32>(0x118ECA8) as usize);
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let slot = *ctx as *mut u32;
        if probe(obj) & 0xFF == 0 {
            *slot = 0;
        } else {
            let obj2 = *table.add(*global::<u32>(0x118ECA8) as usize);
            let flags = *((obj2.wrapping_add(0x5C)) as *const u32);
            *slot = u32::from(flags != 0);
        }
        0
    }
});
