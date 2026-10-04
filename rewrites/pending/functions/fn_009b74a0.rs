// original: 0x009b74a0 rw_009b74a0
/// Forward the index stored at `this+0x52c` to the element-address helper,
/// passing `this` through. Returns the helper's answer.
export!(thiscall, rw_009b74a0(this_: u32) -> u32 {
    unsafe {
        const INDEX_OFF: u32 = 0x52C;
        let idx = *((this_.wrapping_add(INDEX_OFF)) as *const u32);
        let elem: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        elem(this_, idx)
    }
});
