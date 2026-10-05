// original: 0x009e8640 ped_vehicle_task_state (proposed)

/// Classify a ped's vehicle/task state into a small integer code (thiscall,
/// no stack arguments, returns full EAX).
///
/// `this` is a CPed. The entry gate reads the ped's current vehicle
/// (`+0xb30`), a flag byte (`+0x26c`, bit 2 required), the vehicle's model
/// index (signed 16-bit at `+0x2e`, must equal a global wanted-model id) and
/// the vehicle's driver slot (`+0xf50`): when the ped is itself recorded as
/// the driver the result is `READY` at once. Otherwise two scripted checks
/// run: a vehicle-state query on the vehicle object, then a task-presence
/// query (`0x2de`) against the ped's intelligence block (`+0x224`,
/// manager at `+0x2e0`). A positive task answer also yields `READY`.
///
/// Anything that fails the gate, or a negative task answer, falls through to
/// a chain of four probe calls against the ped with kinds 3, 5, 8 and 4.
/// Each probe takes the ped pointer, a pointer to one scratch word and the
/// kind, and answers in AL only (upper bytes ignored, as in the original).
/// The first positive probe wins (`PROBE_A/B/C`); the last probe answers
/// `PROBE_D` when positive and `READY` when negative.
///
/// Every path ends with the CRT security-cookie check, which preserves all
/// registers including the result in EAX.
lf_checker_rt::export!(thiscall, rw_009e8640(this: u32) -> u32 {
    unsafe {
        const CUR_VEHICLE: u32 = 0x0b30;
        const PED_STATE_FLAGS: u32 = 0x026c;
        const FLAG_SEATED: u8 = 0x04;
        const ENTITY_MODEL_IDX: u32 = 0x002e;
        const VEH_DRIVER_PED: u32 = 0x0f50;
        const PED_INTEL: u32 = 0x0224;
        const TASK_MGR_OFF: u32 = 0x02e0;
        const TASK_QUERY_ID: u32 = 0x02de;
        const WANTED_MODEL_GLOB: u32 = 0x012fa47c;
        const READY: u32 = 0x0f;
        const PROBE_A: u32 = 0x0c;
        const PROBE_B: u32 = 0x0d;
        const PROBE_C: u32 = 0x0e;
        const PROBE_D: u32 = 0x0b;
        const CK_COOKIE: u32 = 0;
        const CK_VEH_STATE: u32 = 1;
        const CK_HAS_TASK: u32 = 2;
        const CK_PROBE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u16 {
            unsafe { (a as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn probe(this: u32, slot: &mut u32, kind: u32) -> bool {
            unsafe {
                let r: u32 = lf_checker_rt::callee_cdecl!(
                    CK_PROBE,
                    u32,
                    this,
                    slot as *mut u32 as u32,
                    kind
                );
                r & 0xff != 0
            }
        }
        #[inline(always)]
        unsafe fn probe_chain(this: u32, slot: &mut u32) -> u32 {
            unsafe {
                if probe(this, slot, 3) {
                    return PROBE_A;
                }
                if probe(this, slot, 5) {
                    return PROBE_B;
                }
                if probe(this, slot, 8) {
                    return PROBE_C;
                }
                if probe(this, slot, 4) {
                    PROBE_D
                } else {
                    READY
                }
            }
        }

        let veh = rd32(this.wrapping_add(CUR_VEHICLE));
        let mut slot: u32 = 0;
        let code: u32 = if veh == 0 {
            probe_chain(this, &mut slot)
        } else if rd8(this.wrapping_add(PED_STATE_FLAGS)) & FLAG_SEATED == 0 {
            probe_chain(this, &mut slot)
        } else {
            let model = rd16(veh.wrapping_add(ENTITY_MODEL_IDX)) as i16 as i32;
            let want = rd32(lf_checker_rt::relocated(WANTED_MODEL_GLOB)) as i32;
            if model != want {
                probe_chain(this, &mut slot)
            } else if rd32(veh.wrapping_add(VEH_DRIVER_PED)) == this {
                READY
            } else {
                let seated: u32 =
                    lf_checker_rt::callee_thiscall!(CK_VEH_STATE, u32, veh);
                if seated & 0xff != 0 {
                    probe_chain(this, &mut slot)
                } else {
                    let mgr = rd32(this.wrapping_add(PED_INTEL)).wrapping_add(TASK_MGR_OFF);
                    let has: u32 = lf_checker_rt::callee_thiscall!(
                        CK_HAS_TASK,
                        u32,
                        mgr,
                        TASK_QUERY_ID,
                        0
                    );
                    if has & 0xff != 0 {
                        READY
                    } else {
                        probe_chain(this, &mut slot)
                    }
                }
            }
        };
        let _: u32 = lf_checker_rt::callee_stdcall!(CK_COOKIE, u32,);
        code
    }
});
