// original: 0x00693BA0 comp_foreach_hook2 (proposed)

//
// Calls the hook at vtable slot +0x18 of every element of the array at
// [obj] (count as a 16-bit word at [obj+4], signed loop bound), passing
// each element in ECX and the two arguments through. No return value.
//
// Original: 0x00693BA0 (thiscall, two stack arguments, callee-cleanup).
lf_checker_rt::export!(thiscall, rw_00693BA0(obj: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        const HOOK_SLOT: u32 = 0x18;
        let count = rd16(obj.wrapping_add(4)) as i32;
        if count > 0 {
            let base = rd32(obj);
            let mut i = 0i32;
            while i < count {
                let elem = rd32(base.wrapping_add((i as u32).wrapping_mul(4)));
                let vtable = rd32(elem);
                let hook: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(rd32(vtable.wrapping_add(HOOK_SLOT)) as usize);
                hook(elem, a0, a1);
                i += 1;
            }
        }
        0
    }
});
