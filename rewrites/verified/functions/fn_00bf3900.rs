// original: 0x00bf3900 bind_slot_indexed
/// Tag the slot (0x9b), resolve the entry through the object's hooks (or the
/// fallback slot at +0x100), and forward it to fetch_block_a.
export!(thiscall, rw_bf3900(this: *mut u8, obj: *const u8, a1: u32) -> u32 {
    unsafe {
        *this = 0x9B;
        *this.add(1) = a1 as u8;
        let vtable = *(obj as *const u32);
        let target = *((vtable as *const u8).add(0xA0) as *const u32);
        let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        let first = probe(obj as u32);
        let base = if first == 0 {
            *((obj.add(0x100)) as *const u32)
        } else {
            let second = probe(obj as u32);
            let v2 = *(second as *const u32);
            let t2 = *((v2 as *const u8).add(0xE0) as *const u32);
            let resolve: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(t2 as usize);
            resolve(second)
        };
        let n = a1
            .wrapping_shl(6)
            .wrapping_add(*((base as *const u8).add(0x14) as *const u32));
        callee_thiscall!(3, u32, this as u32, n)
    }
});

#[allow(dead_code)]
fn _use_rt() -> u32 {
    relocated(0x400000)
}
