// original: 0x00981AA0 audio_sector_rebuild
/// Rebuild one audio sector's sample extents and voice table from a bank id.
///
/// Behaviour: resolves the bank, reads the sample count, flag and level
/// through an out-parameter block, allocates the pair array on first use,
/// folds every pair into running minima and maxima, recentres the extents,
/// allocates the voice tables, derives per-voice gains from pair distances
/// and, for quiet sectors, initialises the eight voice slots. Returns the
/// sector width, or 8 after initialising the slots.
export!(thiscall, rw_rb52_981aa0(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let r1: u32 = callee_cdecl!(1, u32, arg, 1);
        // Sample block: count, flag and level arrive through the out-block.
        let mut blk = [0u32; 8];
        let slot1 = blk.as_mut_ptr() as u32;
        let slot2 = (blk.as_mut_ptr() as u32).wrapping_add(12);
        callee_cdecl!(2, u32, r1, relocated(0xE8D8AC), slot1, slot2);
        let count = blk[0];
        let flag = blk[3];
        let level = f32::from_bits(blk[7]);
        // The width below is the block's own count word, reloaded from the
        // same slot the out-call filled (not a register).
        let mut edi = count;
        *(this.add(0x152) as *mut u8) = if flag > 0 { 1 } else { 0 };
        let mut ecx = count;
        if *(this.add(0x32) as *const u16) == 0 {
            *(this.add(0x32) as *mut u16) = edi as u16;
            if edi != 0 {
                let p: u32 = callee_cdecl!(3, u32, edi.wrapping_mul(8));
                edi = count;
                ecx = count;
                *(this.add(0x2C) as *mut u32) = p;
            } else {
                *(this.add(0x2C) as *mut u32) = 0;
            }
        }
        *(this.add(0x30) as *mut u16) = ecx as u16;
        *(this.add(0x13C) as *mut u32) = level.to_bits();
        *(this.add(0x130) as *mut u32) = 0x7F7FFFFF;
        *(this.add(0x134) as *mut u32) = 0x7F7FFFFF;
        *(this.add(0x138) as *mut u32) = 0x7F7FFFFF;
        *(this.add(0x14C) as *mut u32) = level.to_bits();
        *(this.add(0x140) as *mut u32) = 0xFF7FFFFF;
        *(this.add(0x144) as *mut u32) = 0xFF7FFFFF;
        *(this.add(0x148) as *mut u32) = 0xFF7FFFFF;
        // Fold every pair into the running minima and maxima.
        let arr = *(this.add(0x2C) as *const u32);
        let mut i = 0u32;
        while i < edi {
            let ra: u32 = callee_cdecl!(1, u32, arg, 1);
            let slot = arr.wrapping_add(i.wrapping_mul(8));
            callee_cdecl!(2, u32, ra, relocated(0xE8D8B4), slot, slot.wrapping_add(4));
            let x = f32::from_bits(*(slot as *const u32));
            let y = f32::from_bits(*(slot.wrapping_add(4) as *const u32));
            let c130 = f32::from_bits(*(this.add(0x130) as *const u32));
            *(this.add(0x130) as *mut u32) = (if x > c130 { c130 } else { x }).to_bits();
            let c134 = f32::from_bits(*(this.add(0x134) as *const u32));
            *(this.add(0x134) as *mut u32) = (if y > c134 { c134 } else { y }).to_bits();
            let c138 = f32::from_bits(*(this.add(0x138) as *const u32));
            *(this.add(0x138) as *mut u32) =
                (if 0.0 > c138 { c138 } else { 0.0 }).to_bits();
            let c140 = f32::from_bits(*(this.add(0x140) as *const u32));
            *(this.add(0x140) as *mut u32) = (if c140 > x { c140 } else { x }).to_bits();
            let c144 = f32::from_bits(*(this.add(0x144) as *const u32));
            *(this.add(0x144) as *mut u32) = (if c144 > y { c144 } else { y }).to_bits();
            let c148 = f32::from_bits(*(this.add(0x148) as *const u32));
            *(this.add(0x148) as *mut u32) =
                (if c148 > 0.0 { c148 } else { 0.0 }).to_bits();
            i += 1;
        }
        // Recentre the extents and clear the voice header.
        let k = if *(this.add(0x152) as *const u8) == 0 {
            *global::<f32>(0xFE8B38)
        } else {
            *global::<f32>(0xFE8C20)
        };
        let r130 = f32::from_bits(*(this.add(0x130) as *const u32)) - k;
        *(this.add(0x130) as *mut u32) = r130.to_bits();
        let r134 = f32::from_bits(*(this.add(0x134) as *const u32)) - k;
        *(this.add(0x134) as *mut u32) = r134.to_bits();
        let r140 = f32::from_bits(*(this.add(0x140) as *const u32)) + k;
        *(this.add(0x140) as *mut u32) = r140.to_bits();
        let r144 = f32::from_bits(*(this.add(0x144) as *const u32)) + k;
        *(this.add(0x144) as *mut u32) = r144.to_bits();
        *(this.add(0) as *mut u32) = 0;
        *(this.add(4) as *mut u32) = 0;
        *(this.add(0x10) as *mut u32) = 0;
        *(this.add(0x18) as *mut u32) = 0;
        *(this.add(0xC) as *mut u32) = 0;
        *(this.add(0x14) as *mut u32) = 0;
        *(this.add(0x1C) as *mut u32) = *global::<u32>(0x1231780);
        // Second allocation gate (only reachable with a zero width).
        if *(this.add(0x32) as *const u16) == 0 {
            *(this.add(0x32) as *mut u16) = edi as u16;
            if edi != 0 {
                let p: u32 = callee_cdecl!(3, u32, edi.wrapping_mul(8));
                edi = count;
                ecx = i;
                *(this.add(0x2C) as *mut u32) = p;
            } else {
                *(this.add(0x2C) as *mut u32) = 0;
            }
        }
        *(this.add(0x30) as *mut u16) = ecx as u16;
        let n = ecx as u16 as u32;
        // Voice table allocation gate.
        if *(this.add(0x3A) as *const u16) == 0 {
            *(this.add(0x3A) as *mut u16) = edi as u16;
            if edi != 0 {
                let p2: u32 = callee_cdecl!(3, u32, edi.wrapping_mul(4));
                *(this.add(0x34) as *mut u32) = p2;
            } else {
                *(this.add(0x34) as *mut u32) = 0;
            }
        }
        *(this.add(0x38) as *mut u16) = edi as u16;
        let rate = *global::<f32>(0x1038A18);
        *(this.add(0x20) as *mut u32) = rate.to_bits();
        *(this.add(0x24) as *mut u32) = (1.0f32 / rate).to_bits();
        // Per-voice gains from consecutive pair distances.
        let a2 = *(this.add(0x2C) as *const u32);
        let b2 = *(this.add(0x34) as *const u32);
        let kk = *global::<f32>(0xFE870C);
        let mut acc = 0.0f32;
        let mut j = 0u32;
        while j + 1 < n {
            let base = a2.wrapping_add(j.wrapping_mul(8));
            let p0 = f32::from_bits(*(base as *const u32));
            let p1 = f32::from_bits(*(base.wrapping_add(4) as *const u32));
            let q0 = f32::from_bits(*(base.wrapping_add(8) as *const u32));
            let q1 = f32::from_bits(*(base.wrapping_add(12) as *const u32));
            let d0 = p1 - q1;
            let d1 = p0 - q0;
            let dist = (d0 * d0 + d1 * d1).sqrt();
            acc += dist;
            *(b2.wrapping_add(j.wrapping_mul(4)) as *mut u32) =
                (1.0f32 / (dist * kk)).to_bits();
            j += 1;
        }
        *(this.add(0x28) as *mut u32) = acc.to_bits();
        // Quiet sectors get their eight voice slots initialised.
        let exit_eax = if *(this.add(0x152) as *const u8) == 0 {
            let mut k2 = 0u32;
            let mut dd = (this as u32).wrapping_add(0x7C);
            while k2 < 8 {
                *(dd.wrapping_sub(0x40) as *mut u32) = 0;
                *(dd as *mut u32) = 0;
                *((this as u32).wrapping_add(k2).wrapping_add(0x9C) as *mut u8) = 1;
                *(dd.wrapping_add(0x28) as *mut u32) = 0;
                *(dd.wrapping_add(0x68) as *mut u32) = 0;
                let rr: u32 = callee_cdecl!(4, u32, 0xFA0, 0x2EE0);
                *(dd.wrapping_add(0x88) as *mut u32) = rr;
                k2 += 1;
                dd = dd.wrapping_add(4);
            }
            8
        } else {
            n
        };
        *(this.add(0x150) as *mut u16) = 0;
        exit_eax
    }
});
