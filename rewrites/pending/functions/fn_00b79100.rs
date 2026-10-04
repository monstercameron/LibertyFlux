// original: 0x00b79100 CTaskSimpleJumpLaunch::vf16
/// Look up a subtask by tag through the child task's virtual table.
///
/// Reads the child pointer at offset `0x4` (null yields null); asks it for
/// its tag through vtable slot `0xc`; when the tag equals the argument the
/// child itself is returned, otherwise the lookup is forwarded to vtable
/// slot `0x40` with the same argument.
export!(thiscall, rw_00b79100(this: u32, arg: u32) -> u32 {
    let child = unsafe { *((this + 4) as *const u32) };
    if child == 0 {
        return 0;
    }
    let vt = unsafe { *(child as *const u32) };
    let probe: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(*((vt + 0x0c) as *const u32) as usize) };
    if probe(child) == arg {
        return child;
    }
    let forward: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(*((vt + 0x40) as *const u32) as usize) };
    forward(child, arg)
});
