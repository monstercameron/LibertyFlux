// original: 0x0089db40 rage::audOnStopSound::vf7
/// Stop-sound dispatch on an audio object (original 0x0089DB40,
/// merged name `rage::audOnStopSound::vf7`).
///
/// Validates the stop request through the sound callee, snapshots the
/// 24-byte position block, resolves the voice-table row for the object's
/// selector, then issues stop commands for the primary and (when present)
/// secondary and tertiary slots. Returns nonzero on success; the early
/// reject returns the validator's value unchanged, and the success return
/// keeps the low byte set with the high bytes of the last call result.
export!(thiscall, rw_89db40(this: *mut u8, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        let r0: u32 = callee_thiscall!(1, u32, this as u32, a1, a2, a3);
        if (r0 & 0xFF) == 0 {
            return r0;
        }
        *this.add(0x3a) &= 0xf7;
        let node = *((this.add(0x94)) as *const u32) as *const u8;
        if *((this.add(0x88)) as *const u32) != 0xffffffff {
            *((this.add(0x88)) as *mut u32) = 0xffffffff;
        }
        let pos = a3 as *mut u8;
        let w0 = core::ptr::read_unaligned(pos as *const u64);
        let w1 = core::ptr::read_unaligned((pos.add(8)) as *const u64);
        let w2 = core::ptr::read_unaligned((pos.add(16)) as *const u64);
        let mgr = relocated(0x115dc18);
        let r1: u32 = callee_thiscall!(2, u32, mgr, *(node as *const u32), this as u32, a2, a3);
        let stride: u32 = *global::<u32>(0x115d964);
        let base: u32 = *global::<u32>(0x115d988);
        let sel = (*this.add(0x40)) as u32;
        let row_at = base
            .wrapping_add(sel.wrapping_mul(0x6f40))
            .wrapping_add(0x6f10);
        let row = *(row_at as *const u32);
        let idx: u32 = if r1 == 0 {
            0xff
        } else {
            (r1.wrapping_sub(row) / stride) & 0xff
        };
        *this.add(0x48) = idx as u8;
        if idx == 0xff {
            return 0;
        }
        let row2 = *(row_at as *const u32);
        let voice = row2.wrapping_add(idx.wrapping_mul(stride));
        if voice == 0 {
            return 0;
        }
        let mut last: u32 = callee_cdecl!(3, u32, relocated(0xe79fdc), 0);
        *((this.add(0x9c)) as *mut u32) = last;
        if *((node.add(4)) as *const u32) != 0 {
            core::ptr::write_unaligned(pos as *mut u64, w0);
            core::ptr::write_unaligned((pos.add(8)) as *mut u64, w1);
            core::ptr::write_unaligned((pos.add(16)) as *mut u64, w2);
            let r3: u32 =
                callee_thiscall!(2, u32, mgr, *((node.add(4)) as *const u32), this as u32, a2, a3);
            let _r4: u32 = callee_thiscall!(4, u32, this as u32, 1, r3);
            last = callee_cdecl!(3, u32, relocated(0xe79fe4), 0);
            *((this.add(0xa0)) as *mut u32) = last;
        }
        if *((node.add(8)) as *const u32) != 0 {
            core::ptr::write_unaligned(pos as *mut u64, w0);
            core::ptr::write_unaligned((pos.add(8)) as *mut u64, w1);
            core::ptr::write_unaligned((pos.add(16)) as *mut u64, w2);
            let r6: u32 =
                callee_thiscall!(2, u32, mgr, *((node.add(8)) as *const u32), this as u32, a2, a3);
            let _r7: u32 = callee_thiscall!(4, u32, this as u32, 2, r6);
            last = callee_cdecl!(3, u32, relocated(0xe79fec), 0);
        }
        (last & 0xffffff00) | 1
    }
});
