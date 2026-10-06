// original: 0x00693D70 comp_array_alloc_fill (proposed)

//
// Allocates `count` slots through the thread-local allocator (tls slot 0
// -> [+8] -> vtable slot +8, called with (count*4, 0x10, 0)) and fills
// them: a null array is returned as is, and a non-positive count (signed)
// returns the array unfilled. Otherwise each slot is resolved in place:
// slots are zeroed when the registry object at tls [.+4] is missing (note
// it is re-read through a carried register, so after a zeroed slot the
// next iteration reads the new array's own second word instead), when the
// slot is null, or when the intercepted lookup answers -1; else the
// intercepted rebase delta is added, the entry is classified with the
// intercepted classifier, and the hook at +0xC of the classification (a
// planted stub, called with the registry object and the slot) runs. The
// incoming argument slot is reused as scratch for the array pointer on
// both sides. Returns the array.
//
// Original: 0x00693D70 (stdcall, one stack argument).
lf_checker_rt::export!(stdcall, rw_00693D70(count: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        const ALLOC_SLOT: u32 = 0x08;
        const LOOKUP: u32 = 3;
        const REBASE: u32 = 4;
        const CLASSIFY: u32 = 5;
        const HOOK_SLOT: u32 = 0x0C;
        let t = lf_checker_rt::tls_slot(0);
        let h = rd32(t.wrapping_add(8));
        let v = rd32(h);
        let alloc: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(rd32(v.wrapping_add(ALLOC_SLOT)) as usize) };
        let arr = alloc(h, count.wrapping_mul(4), 0x10, 0);
        if (count as i32) <= 0 {
            return arr;
        }
        let mut edi = t;
        let mut slot = arr;
        let mut n = count;
        loop {
            if slot != 0 {
                edi = rd32(edi.wrapping_add(4));
                let mut zero_it = edi == 0;
                if !zero_it {
                    let ebp = rd32(slot);
                    if ebp == 0 {
                        zero_it = true;
                    } else {
                        let r = lf_checker_rt::callee_thiscall!(LOOKUP, u32, rd32(edi), slot);
                        if r == 0xFFFF_FFFF {
                            zero_it = true;
                        } else {
                            let d = lf_checker_rt::callee_thiscall!(REBASE, u32, edi, ebp);
                            let newv = ebp.wrapping_add(d);
                            wr32(slot, newv);
                            let q = rd32(newv.wrapping_add(4));
                            let ans = lf_checker_rt::callee_thiscall!(CLASSIFY, u32, q);
                            let target = rd32(ans.wrapping_add(HOOK_SLOT));
                            edi = t;
                            let m4 = rd32(edi.wrapping_add(4));
                            let hook: extern "cdecl" fn(u32, u32) -> u32 =
                                unsafe { core::mem::transmute(target as usize) };
                            hook(m4, newv);
                        }
                    }
                }
                if zero_it {
                    edi = arr;
                    wr32(slot, 0);
                }
            }
            slot = slot.wrapping_add(4);
            n = n.wrapping_sub(1);
            if n == 0 {
                break;
            }
        }
        arr
    }
});
