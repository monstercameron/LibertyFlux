// original: 0x00a7cf90 euphoria_message_dispatch
/// Builds a message from the incoming arguments plus a selector byte, sends
/// it through the target's dispatch slot and reports the outcome through the
/// owning node.
export!(thiscall, rw_00a7cf90(obj: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        let sel = callee_thiscall!(1, u32, obj);
        let tagged = (obj & 0xFFFFFF00) | (sel & 0xFF);
        let vt = *(a0 as *const u32);
        let send: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(*((vt + 0xbc) as *const u32) as usize);
        let r = send(a0, a1, tagged, a2, a3, 0x20);
        callee_thiscall!(3, u32, obj, r)
    }
});
