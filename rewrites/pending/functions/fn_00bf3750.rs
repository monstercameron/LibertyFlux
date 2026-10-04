// original: 0x00bf3750 sync_entity_transform (proposed name)
/// Synchronize an entity's transform, flags and record fields.
///
/// `this_` carries option bytes (bit 0x10 at +8 selects the record
/// block, bytes +0xd/+0xe/+0xf/+0x18 feed helpers), `entity` the entity,
/// `arg2` an auxiliary object and `weight` a blend weight. A null entity
/// returns immediately (its incoming-register result is not modeled; the
/// contract never passes null). Otherwise a preparation step runs (full
/// blend, simple helper, or nothing, depending on the update bit, the
/// weight and two probe answers), then the record block, a virtual
/// setter, a flag splice, an optional gated virtual call, a matrix
/// refresh and a final virtual call whose answer is the result.
export!(thiscall, rw_00bf3750(this_: *mut u8, entity: *mut u8, arg2: u32, weight: f32) -> u32 {
    unsafe {
        let bit_set = (*(entity.add(0x24) as *const u32) & 0x400) != 0;
        if bit_set || weight == 0.0 {
            let mut s3 = [0u32; 4];
            callee_thiscall!(4, u32, this_ as u32, s3.as_mut_ptr() as u32);
        } else {
            let a1 = callee_thiscall!(1, u32, this_ as u32);
            if a1 == 0xffffff {
                let mut s3 = [0u32; 4];
                callee_thiscall!(4, u32, this_ as u32, s3.as_mut_ptr() as u32);
            } else {
                let a2 = callee_thiscall!(1, u32, this_ as u32);
                if a2 != 0 && arg2 != 0 {
                    let mut s1 = [0u32; 4];
                    let mut s2 = [0u32; 4];
                    let mut s3 = [0u32; 4];
                    callee_thiscall!(2, u32, arg2, s2.as_mut_ptr() as u32, s1.as_mut_ptr() as u32);
                    callee_thiscall!(3, u32, this_ as u32, s3.as_mut_ptr() as u32,
                        s2.as_mut_ptr() as u32, s1.as_mut_ptr() as u32, weight.to_bits());
                }
            }
        }
        let b8 = *(this_.add(8) as *const u8);
        if (b8 & 0x10) != 0 {
            let p210 = entity.add(0x210) as *mut u32;
            let mut t = ((b8 as u32) << 23) ^ p210.read();
            t &= 0x08000000;
            p210.write(p210.read() ^ t);
            *(entity.add(0x240) as *mut u32) = *(this_.add(0x14) as *const u32);
            let d = *(this_.add(0x0d) as *const u8) as u32;
            let e = *(this_.add(0x0e) as *const u8) as u32;
            let f = *(this_.add(0x0f) as *const u8) as u32;
            let r1 = callee_cdecl!(5, u32, f, d);
            *(entity.add(0x244) as *mut u32) = r1;
            let r2 = callee_cdecl!(6, u32, f, d, e);
            *(entity.add(0x248) as *mut u32) = r2;
        }
        let mut sf = [0u32; 4];
        callee_thiscall!(7, u32, entity as u32, sf.as_mut_ptr() as u32, 0, 0);
        *(entity.add(0x63) as *mut u8) = *(this_.add(9) as *const u8);
        let vt = *(entity as *const u32);
        let slot_a = ((vt + 0x30) as *const u32).read();
        let vcall_a: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot_a as usize);
        vcall_a(entity as u32, (b8 & 1) as u32);
        let p24 = entity.add(0x24) as *mut u32;
        let flip = (((b8 as u32) << 6) ^ p24.read()) & 0x100;
        p24.write(p24.read() ^ flip);
        let gate = *global::<u32>(0x011F70D4);
        if gate != 2 {
            let cx = entity.add(0x80);
            let vtb = *(cx as *const u32);
            *(cx.add(0x3c) as *mut u8) = 0;
            let slot_b = ((vtb + 8) as *const u32).read();
            let vcall_b: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(slot_b as usize);
            vcall_b(cx as u32, 0, *(this_.add(0x18) as *const u8) as u32);
        }
        callee_thiscall!(10, u32, entity as u32);
        let slot_c = ((vt + 0xb4) as *const u32).read();
        let vcall_c: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot_c as usize);
        vcall_c(entity as u32, 1)
    }
});
