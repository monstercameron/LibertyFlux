// original: 0x00bf3c40 route_slot
/// Route the slot through one of two emitters based on the object flags and
/// the float argument, then run the epilogue pair when `flag` is set.
/// Early exits return 0 here; the original returns entry EAX (or EAX with a
/// flags byte folded in) on two of those paths, which no caller can observe
/// meaningfully, so the return channel is none.
export!(thiscall, rw_bf3c40(this: *mut u8, obj: u32, aux: u32, f2bits: u32, flag: u32) -> u32 {
    unsafe {
        if obj == 0 {
            return 0;
        }
        if (*this.add(1) as i8) < 0 {
            return 0;
        }
        let vtable = *(obj as *const u32);
        let target = *((vtable as *const u8).add(0xA0) as *const u32);
        let probe: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
        let inner = probe(obj);
        if inner == 0 {
            return 0;
        }
        let grp = *((inner as *const u8).add(0x64) as *const u32);
        if grp == 0 {
            return 0;
        }
        let tab = if *this.add(2) == 0 {
            *((grp as *const u8).add(0x160) as *const u32)
        } else {
            *((grp as *const u8).add(0x168) as *const u32)
        };
        let stride = ((*this.add(1) as i8) as i32 as u32).wrapping_shl(6);
        let n = stride.wrapping_add(*((tab as *const u8).add(0x14) as *const u32));
        let threshold = f32::from_bits(*global::<u32>(0xFE8628));
        let mut ans = 0u32;
        let flagged = (*((obj as *const u8).add(0x24) as *const u32) & 0x400) != 0;
        if !flagged && f32::from_bits(f2bits) != threshold {
            if aux != 0 {
                let mut b1 = [0u32; 4];
                let mut b2 = [0u32; 4];
                let _: u32 =
                    callee_thiscall!(2, u32, aux, b2.as_mut_ptr() as u32, b1.as_mut_ptr() as u32);
                let mut c1 = [0u32; 4];
                let mut c2 = [0u32; 4];
                ans = callee_thiscall!(
                    3, u32, this as u32, n,
                    c2.as_mut_ptr() as u32, c1.as_mut_ptr() as u32, f2bits
                );
            }
        } else {
            ans = callee_thiscall!(4, u32, this as u32, n);
        }
        if (flag as u8) != 0 {
            let _: u32 = callee_cdecl!(5, u32, obj, 0);
            ans = callee_cdecl!(6, u32, obj, 1);
        }
        ans
    }
});
