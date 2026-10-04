// original: 0x008976e0 MUSIC
/// Initialise a music-state object (thiscall, no stack args).
///
/// Readiness gate first (thiscall/0, stubbed as id 1): a zero low byte
/// returns the gate answer with its low byte cleared. Otherwise zero the
/// 136-byte state block, then make nine descriptor-driven setup calls to one
/// helper (thiscall/3, one stub id per call site, ids 2-10) sharing a single
/// 8-word stack parameter block whose address argument is skipped and whose
/// contents are snapshotted (site 6's third word holds a stack address, so
/// only its first two words are snapshotted). Finally count the leading set
/// entries among the nine dwords at +0x64: 1 when all nine are set, else the
/// count. Touches no globals; float constants move by bits only.
export!(thiscall, rw_008976e0(this: *mut u8) -> u32 {
    unsafe {
        let obj = this;
        let r1 = callee_thiscall!(1, u32, obj.add(0x88) as u32);
        if r1 & 0xFF == 0 {
            return r1 & 0xFFFFFF00;
        }
        for i in 0..34u32 {
            *(obj.add((i * 4) as usize) as *mut u32) = 0;
        }
        const ONE: u32 = 0x3F800000; // 1.0f by bits
        let mut b = [0u32; 8];
        let bp = b.as_mut_ptr() as u32;
        // Site 1.
        b[2] = 0;
        b[3] = ONE;
        b[4] = ONE;
        b[5] = ONE;
        b[6] = ONE;
        b[7] = ONE;
        *(bp as *mut u16) = 0x0105;
        b[1] = 8;
        callee_thiscall!(2, u32, obj as u32, relocated(0x00E78F9C), obj.add(0x84) as u32, bp);
        // Site 2.
        *(bp as *mut u16) = 0x0104;
        b[1] = 7;
        b[2] = obj.add(0x84) as u32;
        callee_thiscall!(3, u32, obj as u32, relocated(0x00E78FA4), obj.add(0x80) as u32, bp);
        // Site 3.
        *(bp as *mut u8) = 3;
        b[1] = 3;
        b[2] = obj.add(0x84) as u32;
        callee_thiscall!(4, u32, obj as u32, relocated(0x00E78FA8), obj.add(0x70) as u32, bp);
        // Site 4.
        *(bp as *mut u8) = 3;
        b[1] = 4;
        b[2] = obj.add(0x80) as u32;
        callee_thiscall!(5, u32, obj as u32, relocated(0x00E78FB0), obj.add(0x74) as u32, bp);
        // Site 5.
        *(bp as *mut u8) = 3;
        b[1] = 6;
        b[2] = obj.add(0x80) as u32;
        callee_thiscall!(6, u32, obj as u32, relocated(0x00E78FB4), obj.add(0x7C) as u32, bp);
        // Site 6: the block's third word points at two scratch words holding
        // state entries (a stack address, unverifiable differentially).
        let scratch = [
            *(obj.add(0x7C) as *const u32),
            *(obj.add(0x80) as *const u32),
        ];
        b[2] = scratch.as_ptr() as u32;
        *(bp as *mut u16) = 0x0202;
        b[1] = 5;
        b[5] = ONE;
        b[6] = ONE;
        callee_thiscall!(7, u32, obj as u32, relocated(0x00E78FC0), obj.add(0x78) as u32, bp);
        // Site 7.
        b[3] = ONE;
        b[4] = ONE;
        b[5] = ONE;
        b[6] = ONE;
        b[7] = ONE;
        *(bp as *mut u16) = 0x0101;
        b[1] = 0;
        b[2] = obj.add(0x78) as u32;
        callee_thiscall!(8, u32, obj as u32, relocated(0x00E78FD0), obj.add(0x64) as u32, bp);
        // Site 8.
        *(bp as *mut u8) = 1;
        b[1] = 1;
        b[2] = obj.add(0x78) as u32;
        callee_thiscall!(9, u32, obj as u32, relocated(0x00E78FE4), obj.add(0x68) as u32, bp);
        // Site 9.
        *(bp as *mut u8) = 1;
        b[1] = 2;
        b[2] = obj.add(0x78) as u32;
        callee_thiscall!(10, u32, obj as u32, relocated(0x00E78FFC), obj.add(0x6C) as u32, bp);
        // Count leading set entries among the nine dwords at +0x64.
        let mut n: u32 = 0;
        let mut p = obj.add(0x64) as *const u32;
        loop {
            if *p == 0 {
                break;
            }
            n += 1;
            p = p.add(1);
            if n >= 9 {
                break;
            }
        }
        if n == 9 {
            1
        } else {
            n
        }
    }
});
