// original: 0x008a6120 rage::audMathOperationSound::vf7
/// Bind a sound to a table entry selected by a helper lookup.
///
/// Forwards the three arguments to a validator and returns its value when its
/// low byte is 0. Otherwise asks a finder (keyed by the head word of the
/// descriptor at +0x94) for an entry address, converts it to a table index by
/// subtracting the slot base and dividing by the stride (0xff when the finder
/// returns null), and stores it at +0x48. Then requires stride room for the
/// descriptor's unit count (*24); allocates through the table owner; converts
/// that address the same way into +0xb5; finishes through a finalizer with
/// the descriptor; returns 1 on success (low byte forced) and 0 on failure.
export!(thiscall, rw_008a6120(this: *mut u8, arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        let r1: u32 = callee_thiscall!(1, u32, this as u32, arg0, arg1, arg2);
        if (r1 & 0xff) == 0 {
            return r1;
        }
        let desc = *(this.add(0x94) as *const u32);
        let head = *(desc as *const u32);
        let r2: u32 = callee_thiscall!(
            2,
            u32,
            relocated(0x115dc18),
            head,
            this as u32,
            arg1,
            arg2
        );
        let index = *this.add(0x40) as u32;
        let stride = *global::<u32>(0x115d964);
        let table = *global::<u32>(0x115d988);
        let slot = table
            .wrapping_add(index.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10) as *const u32;
        let base = *slot;
        if r2 == 0 {
            *this.add(0x48) = 0xff;
        } else {
            *this.add(0x48) = r2.wrapping_sub(base).wrapping_div(stride) as u8;
        }
        let units = *((desc as *const u8).add(4));
        let need = (units as u32).wrapping_mul(3).wrapping_mul(8);
        if stride < need {
            return 0;
        }
        let r3: u32 = callee_thiscall!(3, u32, relocated(0x115d8a0), need, index, 1);
        if r3 == 0 {
            return 0;
        }
        *this.add(0xb5) = r3.wrapping_sub(base).wrapping_div(stride) as u8;
        let r4: u32 = callee_thiscall!(4, u32, this as u32, desc);
        (r4 & 0xffffff00) | 1
    }
});
