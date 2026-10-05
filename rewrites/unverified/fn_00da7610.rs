// original: 0x00DA7610 CTaskComplexSmartFleeEntity::vf20

/// Refresh the smart-flee task against the ped's current threat.
///
/// `this` is the task, `ped` the ped. The ped is marked fleeing
/// (`+0x2A0` bit `0x20`) up front. When the task has no threat holder
/// (`+0x14` null) it runs the subtask's slot-`0x14` check with the ped and
/// either flags the subtask done (bit 2 of `+0xC`) or leaves it; either way
/// the subtask pointer is returned.
///
/// Otherwise the task may run a 10-argument setup call (only when the
/// `+0x38` flag is set and the ped's kind-table entry byte at `+0xEE` is
/// below 3), then requires the subtask's slot-`0xC` answer to be `0x38E`,
/// stores `+0x44` into the subtask's `+0x58`, and requires the `+0x50` flag.
/// A time gate follows: unless `+0x48 + +0x4C` is still (signed) at or
/// below the clock global the subtask is returned unchanged (`+0x51` set
/// re-arms the gate from the clock first). Then the squared distance from
/// the task point (`+0x20/+0x24/+0x28`) to the threat point (the holder's
/// `+0x20` plus `0x30`, or the holder's `+0x10` when null) must exceed the
/// squared radius at `+0x40` (dy squared plus dx squared, plus dz squared;
/// "not above" exits, so NaN exits).
///
/// Past the distance gate the task re-arms its time window from the clock
/// and `+0x3C`, sets `+0x50`, copies the threat point over its own
/// `+0x20..+0x2C`, and issues the flee call with its point, the `+0x34`
/// float and the `+0x38` flag. When `+0x44` exceeds 2 it also builds a
/// four-word stack record through one callee, hands it to two more, and
/// tears it down through a fourth; the record's address is skipped in the
/// call comparison while its scripted contents are snapshotted.
///
/// The subtask pointer is returned on every path.
///
/// Original: 0x00DA7610 (thiscall, one stack word, returns eax).
lf_checker_rt::export!(thiscall, rw_00DA7610(this: u32, ped: u32) -> u32 {
    unsafe {
        const PED_FLEE_BIT: u32 = 0x2A0;
        const PED_KIND: u32 = 0x2E;
        const PED_BLOCK_OFF: u32 = 0x570;
        const KIND_TABLE: u32 = 0x01295CD8;
        const KIND_LIMIT_OFF: u32 = 0xEE;
        const SETUP_BLOCK: u32 = 0x00EEF9B4;
        const SETUP_FLAG: u32 = 0x01284530;
        const CLOCK: u32 = 0x011735B4;
        const TASK_HOLDER: u32 = 0x14;
        const TASK_SUB: u32 = 0x8;
        const TASK_POINT_X: u32 = 0x20;
        const TASK_POINT_Y: u32 = 0x24;
        const TASK_POINT_Z: u32 = 0x28;
        const TASK_REACH: u32 = 0x34;
        const TASK_FLAG: u32 = 0x38;
        const TASK_RADIUS: u32 = 0x40;
        const TASK_MODE: u32 = 0x44;
        const TASK_T0: u32 = 0x48;
        const TASK_SPAN: u32 = 0x4C;
        const TASK_ARMED: u32 = 0x50;
        const TASK_REARM: u32 = 0x51;
        const TASK_WINDOW: u32 = 0x3C;
        const HOLDER_POS: u32 = 0x20;
        const HOLDER_FALLBACK: u32 = 0x10;
        const POS_OFF: u32 = 0x30;
        const ONE_BITS: u32 = 0x3F800000;
        const WANT_KIND: u32 = 0x38E;
        const VT_SLOT_KIND: u32 = 0x0C;
        const VT_SLOT_CHECK: u32 = 0x14;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn fsub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        wr32(
            ped.wrapping_add(PED_FLEE_BIT),
            rd32(ped.wrapping_add(PED_FLEE_BIT)) | 0x20,
        );
        let sub: u32 = rd32(this.wrapping_add(TASK_SUB));
        let holder: u32 = rd32(this.wrapping_add(TASK_HOLDER));
        if holder == 0 {
            if (sub.wrapping_add(0x0C) as *const u8).read() & 1 != 0 {
                return sub;
            }
            let vt: u32 = rd32(sub);
            let check: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_CHECK)) as usize);
            if check(sub, ped, 0, 0) & 0xFF == 0 {
                return sub;
            }
            ((sub.wrapping_add(0x0C)) as *mut u8)
                .write((sub.wrapping_add(0x0C) as *const u8).read() | 2);
            return sub;
        }

        if (this.wrapping_add(TASK_FLAG) as *const u8).read() != 0 {
            let kind: u32 =
                ((ped.wrapping_add(PED_KIND) as *const i16).read_unaligned() as i32) as u32;
            let table: u32 = lf_checker_rt::relocated(KIND_TABLE);
            let entry: u32 = rd32(table.wrapping_add(kind.wrapping_mul(4)));
            if (entry.wrapping_add(KIND_LIMIT_OFF) as *const u8).read() < 3 {
                let block: u32 = lf_checker_rt::relocated(SETUP_BLOCK);
                let flag: u32 = lf_checker_rt::global::<u32>(SETUP_FLAG).read();
                let _ = lf_checker_rt::callee_thiscall!(
                    3, u32, ped.wrapping_add(PED_BLOCK_OFF),
                    block, 0, 1, flag, 0xFFFF_FFFF, 0, 0, ONE_BITS, 0, 0
                );
            }
        }

        let vt: u32 = rd32(sub);
        let kind_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(vt.wrapping_add(VT_SLOT_KIND)) as usize);
        if kind_of(sub) != WANT_KIND {
            return sub;
        }
        wr32(sub.wrapping_add(0x58), rd32(this.wrapping_add(TASK_MODE)));
        if (this.wrapping_add(TASK_ARMED) as *const u8).read() == 0 {
            return sub;
        }
        if (this.wrapping_add(TASK_REARM) as *const u8).read() != 0 {
            wr32(
                this.wrapping_add(TASK_T0),
                lf_checker_rt::global::<u32>(CLOCK).read(),
            );
            (this.wrapping_add(TASK_REARM) as *mut u8).write(0);
        }
        let now: u32 = lf_checker_rt::global::<u32>(CLOCK).read();
        let end: u32 = rd32(this.wrapping_add(TASK_T0)).wrapping_add(rd32(this.wrapping_add(TASK_SPAN)));
        if (end as i32) > (now as i32) {
            return sub;
        }

        let threat: u32 = {
            let p: u32 = rd32(holder.wrapping_add(HOLDER_POS));
            if p != 0 {
                p.wrapping_add(POS_OFF)
            } else {
                holder.wrapping_add(HOLDER_FALLBACK)
            }
        };
        let dx: f32 = fsub(rdf(this.wrapping_add(TASK_POINT_X)), rdf(threat));
        let dy: f32 = fsub(rdf(this.wrapping_add(TASK_POINT_Y)), rdf(threat.wrapping_add(4)));
        let dz: f32 = fsub(rdf(this.wrapping_add(TASK_POINT_Z)), rdf(threat.wrapping_add(8)));
        let radius: f32 = rdf(this.wrapping_add(TASK_RADIUS));
        let dist2: f32 = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz));
        let rad2: f32 = mul(radius, radius);
        if !(dist2 > rad2) {
            return sub;
        }

        wr32(this.wrapping_add(TASK_T0), now);
        wr32(this.wrapping_add(TASK_SPAN), rd32(this.wrapping_add(TASK_WINDOW)));
        (this.wrapping_add(TASK_ARMED) as *mut u8).write(1);
        let threat2: u32 = {
            let h: u32 = rd32(this.wrapping_add(TASK_HOLDER));
            let p: u32 = rd32(h.wrapping_add(HOLDER_POS));
            if p != 0 {
                p.wrapping_add(POS_OFF)
            } else {
                h.wrapping_add(HOLDER_FALLBACK)
            }
        };
        for off in [0u32, 4, 8, 12] {
            wr32(this.wrapping_add(TASK_POINT_X).wrapping_add(off), rd32(threat2.wrapping_add(off)));
        }
        let _ = lf_checker_rt::callee_thiscall!(
            4, u32, sub, this.wrapping_add(TASK_POINT_X),
            rd32(this.wrapping_add(TASK_REACH)),
            (this.wrapping_add(TASK_FLAG) as *const u8).read() as u32
        );
        if (rd32(this.wrapping_add(TASK_MODE)) as i32) <= 2 {
            return sub;
        }
        let mut record: [u32; 4] = [0, 0, 0, 0];
        let rec_addr: u32 = (&mut record as *mut u32) as u32;
        let _ = lf_checker_rt::callee_thiscall!(5, u32, rec_addr, ped);
        let token: u32 = lf_checker_rt::callee_stdcall!(6, u32, rec_addr, 0, 1);
        let _ = lf_checker_rt::callee_thiscall!(7, u32, token);
        let _ = lf_checker_rt::callee_thiscall!(8, u32, rec_addr);
        sub
    }
});
