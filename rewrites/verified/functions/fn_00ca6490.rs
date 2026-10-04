// original: 0x00ca6490 CEventHandler::vf43
/// Event reaction 43: poll the owner's task slot, then either take the
/// behaviour the manager offers straight away or, when the owner idles,
/// fetch a fresh pose through a virtual handoff and store it.
///
/// The busy flag polled from the task slot travels into the behaviour
/// request. When the manager answers on the idle path, the returned record
/// receives four pose words plus a ready marker; the record (or zero when
/// no manager answers) is stored at owner+0xc.
export!(thiscall, rw_rb02_vf43(this_ptr: u32, _a1: u32, _a2: u32, _a3: u32) -> u32 {
    unsafe {
        let p1 = *((this_ptr.wrapping_add(4)) as *const u32);
        let status = (p1.wrapping_add(0x26c)) as *mut u32;
        *status &= 0xffffdfff;
        *((p1.wrapping_add(0x118)) as *mut u32) |= 1;
        *status &= 0xfffffffe;
        let slot = *((p1.wrapping_add(0x224)) as *const u32);
        let busy: u32 = callee_thiscall!(1, u32, slot.wrapping_add(0x44), 0x2deu32);
        let flag: u32 = if busy != 0 { 1 } else { 0 };
        let bus = *global::<u32>(0x167e2a0);
        if *((p1.wrapping_add(0x219)) as *const u8) != 0 {
            let mgr: u32 = callee_thiscall!(2, u32, bus);
            if mgr == 0 {
                *((this_ptr.wrapping_add(0xc)) as *mut u32) = 0;
                return 0;
            }
            let ans: u32 = callee_thiscall!(3, u32, mgr, 0u32, 0u32, flag);
            *((this_ptr.wrapping_add(0xc)) as *mut u32) = ans;
            return ans;
        }
        let mgr2: u32 = callee_thiscall!(2, u32, bus);
        let esi: u32 = if mgr2 == 0 {
            0
        } else {
            callee_thiscall!(3, u32, mgr2, 0u32, 0u32, flag)
        };
        if *((p1.wrapping_add(0xb80)) as *const u32) == 1 {
            *((this_ptr.wrapping_add(0xc)) as *mut u32) = esi;
            return p1;
        }
        let p: u32 = callee_thiscall!(4, u32, slot.wrapping_add(0x44));
        if p == 0 {
            *((this_ptr.wrapping_add(0xc)) as *mut u32) = esi;
            return 0;
        }
        // Virtual handoff: fetch the pose writer, gate on its answer.
        let vt = *(p as *const u32);
        let tgt_a = *((vt.wrapping_add(0x30)) as *const u32);
        let fetch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(tgt_a as usize);
        let q: u32 = fetch(p);
        let w = *(q as *const u32);
        let tgt_b = *((w.wrapping_add(0x18)) as *const u32);
        let gate: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(tgt_b as usize);
        let ans_b: u32 = gate(q);
        if (ans_b & 0xff) == 0 {
            *((this_ptr.wrapping_add(0xc)) as *mut u32) = esi;
            return ans_b;
        }
        let mut pose = [0u32; 4];
        let tgt_c = *((w.wrapping_add(0x1c)) as *const u32);
        let fill: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(tgt_c as usize);
        let ans_c: u32 = fill(q, (&mut pose as *mut u32) as u32);
        *((esi.wrapping_add(0x20)) as *mut u32) = pose[0];
        *((esi.wrapping_add(0x24)) as *mut u32) = pose[1];
        *((esi.wrapping_add(0x28)) as *mut u32) = pose[2];
        *((esi.wrapping_add(0x2c)) as *mut u32) = pose[3];
        *((esi.wrapping_add(0x68)) as *mut u8) = 1;
        *((this_ptr.wrapping_add(0xc)) as *mut u32) = esi;
        ans_c
    }
});
