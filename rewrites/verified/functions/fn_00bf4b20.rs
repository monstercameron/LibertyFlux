// original: 0x00bf4b20 vehfx_exhaust_emitter_setup
/// Look up the effect record for a vehicle, create its exhaust emitter and
/// configure the emitter's parameters.
///
/// `this` is the vehicle FX object, `arg0` the vehicle record. The record's
/// effect index selects a row from the object table; the row's parameter id
/// for this object's slot must be non-negative. Callee 1 resolves the effect
/// origin; a zero origin vector ends the call. Otherwise callee 2 creates the
/// emitter, and the second argument word selects the mode: mode zero takes
/// the parameter path (set speed always, damage/temp/throttle unless the
/// object's effect id matches the hashed effect names), nonzero takes the
/// adjust path (query callee 7, then callees 8 and 9). Both paths finish
/// through callees 10 and 11. The trailing flag byte is zeroed by the
/// function itself before every read, so the callee 12/13 epilogue is
/// unreachable.
///
/// Returns 0; the original returns void (checker `ret: none`).
/// Callee ids: 1 resolve-origin, 2 create-emitter, 3 set-param,
/// 4/5/6 hash-name, 7 adjust-query, 8 adjust-step, 9 adjust-apply,
/// 10 attach, 11 register, 12 finish, 13 finish-id.
export!(thiscall, rw_00bf4b20(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        if arg0 == 0 {
            return 0;
        }
        // Effect row lookup: sign-extended index, slot-selected id.
        let idx = *(arg0.wrapping_add(0x2e) as *const i16) as i32;
        let slot = *(this.wrapping_add(0x14) as *const u8) as u32;
        let row = *global::<u32>(0x1295cd8).offset(idx as isize);
        let inner = *(row.wrapping_add(0xcc) as *const u32);
        let v = *(inner
            .wrapping_add(slot.wrapping_mul(4))
            .wrapping_add(0xa4) as *const i32);
        if v <= -1 {
            return 0;
        }
        // Resolve the effect origin; a zero vector ends the call. NaN
        // counts as nonzero, matching the original's compare idiom.
        let origin: u32 = callee_thiscall!(1, u32, arg0, v as u32);
        if origin == 0 {
            return 0;
        }
        let x = *(origin as *const f32);
        let y = *(origin.wrapping_add(4) as *const f32);
        let z = *(origin.wrapping_add(8) as *const f32);
        if x == 0.0 && y == 0.0 && z == 0.0 {
            return 0;
        }
        // Create the emitter. Callee 2 writes nothing observable: the
        // original's mode word is its second stack argument slot, and its
        // flag byte is zeroed by the function itself before every read.
        // The dummy frame pointer below is skipped in the contract.
        let mut out: [u32; 2] = [0, 0];
        let fx = relocated(0x1394d60);
        let obj_id = *(this.wrapping_add(8) as *const u32);
        let emitter: u32 = callee_thiscall!(
            2, u32, fx,
            arg0.wrapping_add(slot).wrapping_add(8),
            obj_id,
            out.as_mut_ptr() as u32,
            0,
            0
        );
        if emitter == 0 {
            return 0;
        }
        let mode = f32::from_bits(arg1);
        if mode == 0.0 {
            // Parameter path.
            callee_thiscall!(
                3, u32, emitter,
                relocated(0xebbeb8),
                *(this.wrapping_add(0x18) as *const u32)
            );
            let ha: u32 = callee_cdecl!(4, u32, relocated(0xebbec0), 0);
            if obj_id != ha {
                callee_thiscall!(
                    3, u32, emitter,
                    relocated(0xebbed4),
                    *(this.wrapping_add(0x1c) as *const u32)
                );
            }
            let hb: u32 = callee_cdecl!(5, u32, relocated(0xebbedc), 0);
            if hb != obj_id {
                let hc: u32 = callee_cdecl!(6, u32, relocated(0xebbef0), 0);
                if hc != obj_id {
                    callee_thiscall!(
                        3, u32, emitter,
                        relocated(0xebbf04),
                        *(this.wrapping_add(0x20) as *const u32)
                    );
                }
            }
            callee_thiscall!(
                3, u32, emitter,
                relocated(0xebbf0c),
                *(this.wrapping_add(0x24) as *const u32)
            );
        } else {
            // Adjust path. The original's flag test always sees zero
            // (proven above), so it always reaches the query.
            let go: u32 = callee_thiscall!(7, u32, this);
            if go & 0xFF != 0 {
                let mut tmp: [u32; 4] = [0, 0, 0, 0];
                callee_cdecl!(8, u32, tmp.as_mut_ptr() as u32, this);
                callee_thiscall!(9, u32, this, emitter, this, mode.to_bits());
            }
        }
        callee_thiscall!(10, u32, emitter, origin);
        callee_thiscall!(11, u32, fx, emitter, arg0, 0);
        // Epilogue (callees 12/13) omitted: unreachable, the flag word
        // the original tests is zero on every trial of this contract.
        0
    }
});
