// original: 0x00698570 rage::crCreatureComponentMover::~crCreatureComponentMover__deleting
/// Forward a mover notification with the component's id word.
///
/// Passes the zero-extended word at offset 0x50 along with the two stack
/// arguments to the shared notification handler. Returns its answer.
export!(thiscall, rw_00698570(this: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        let id = *((this as *const u16).add(0x28)) as u32;
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        f(arg1, id, 0, arg2)
    }
});
