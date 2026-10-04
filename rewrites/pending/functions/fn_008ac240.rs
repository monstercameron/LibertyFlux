// original: 0x008AC240 rage::audReverbEffect::vf5
/// Advance one tick: shuffle one parameter row, refresh the four live
/// parameters from the preset table wherever the stored value has risen
/// past the floor or the hold flag is clear, then hand control to the
/// linked stage, or report the step counter when unlinked.
export!(thiscall, rw_008AC240(obj: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, obj as u32);
        let step = *(obj.add(0x30) as *const u32);
        let slot = step.wrapping_add(1) % 3;
        let src = step.wrapping_mul(5).wrapping_mul(4);
        let dst = slot.wrapping_mul(5).wrapping_mul(4);
        for i in 0..2u32 {
            let v = core::ptr::read_unaligned(
                (obj as *const u8)
                    .add(src.wrapping_add(0x74).wrapping_add(i * 8) as usize)
                    as *const u64,
            );
            core::ptr::write_unaligned(
                (obj as *mut u8)
                    .add(dst.wrapping_add(0x74).wrapping_add(i * 8) as usize)
                    as *mut u64,
                v,
            );
        }
        let tail = *(obj.add(src.wrapping_add(0x84) as usize) as *const u32);
        *(obj.add(dst.wrapping_add(0x84) as usize) as *mut u32) = tail;
        let floor = *(global::<f32>(0x00FE8D7C) as *const f32);
        let table = *(obj.add(4) as *const u32);
        // NOTE: the hold flag is re-read per block: block1's store target
        // aliases it when step == 4 (20*4+0x74 == 0xC4).
        let refresh = |off: u32, src_off: usize| -> bool {
            let mem = *(obj.add(off as usize) as *const f32);
            if floor > mem || *obj.add(0xC4) == 0 {
                let v = *((table as *const u8).add(src_off) as *const u32);
                *(obj.add(off as usize) as *mut u32) = v;
                return true;
            }
            false
        };
        let base = step.wrapping_mul(5).wrapping_mul(4);
        refresh(base.wrapping_add(0x74), 0x0F);
        let grown = step.wrapping_add(6).wrapping_mul(5).wrapping_mul(4);
        refresh(grown, 0x13);
        refresh(base.wrapping_add(0x7C), 0x17);
        let last = base.wrapping_add(0x80);
        let last_stored = refresh(last, 0x1B);
        // EAX at the fall-through exit: the last stored value, or the
        // step counter when the last block did not store.
        let tail_eax = if last_stored {
            *(obj.add(last as usize) as *const u32)
        } else {
            step
        };
        let next = *(obj.add(8) as *const u32);
        *(obj.add(0x30) as *mut u32) = slot;
        if next == 0 {
            return tail_eax;
        }
        let vtable = *(next as *const u32);
        let target = *((vtable as *const u8).add(0x14) as *const u32);
        let advance: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(target as usize);
        advance(next)
    }
});
