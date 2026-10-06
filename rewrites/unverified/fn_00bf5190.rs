// original: 0x00bf5190 vehfx_overheat_emitter_setup
/// Look up the effect record for a vehicle, create its overheat emitter and
/// configure the emitter's parameters.
///
/// `this` is the vehicle FX object, `arg0` the vehicle record, `arg1` a mode
/// word read as float. A null record ends the call. Nine hashed effect names
/// split into three table paths: the first two select the plus-0xB4 word, the
/// next seven select the plus-0xB8/0xBC word by the flag bit, otherwise the
/// mode is -1. A non-negative table word resolves an origin through callee 10
/// (a zeroed slot otherwise). Callee 11 creates the emitter from the record
/// pointer plus the mode; a null emitter ends the call.
///
/// A zero mode word takes the parameter path (three parameter sets through
/// callee 12, then nine more hash gates choosing an optional fourth set,
/// then a conditional flag store at plus-0x1E2); any other mode word,
/// including NaN, takes the adjust path (flag-gated, then callee 22, then
/// callees 23 and 24). Both paths finish through the attach/register spine
/// (callees 25 and 26), the record's virtual aim hook in slot 0xec (whose
/// three answer words become the emitter triple at plus-0x190), and a
/// flag-gated finish (callees 28 and 29 plus a global compare into
/// plus-0x1D4).
///
/// Callee 24's first stack word reads a frame slot the original never stores
/// to; the contract pins the stack fill to zero and the rewrite passes a
/// literal zero for it. Frame-pointer arguments (create slot, callee 23
/// out-word, aim-hook input) are skipped in the comparison with call-time
/// snapshots; the create flag and the callee 23 out-word are verified through
/// the branches and calls that consume them, and the aim-hook input is never
/// read back.
///
/// Returns 0; the original returns void (checker `ret: none`).
/// Callee ids: 1-9 first hash chain, 10 resolve-origin, 11 create-emitter,
/// 12 set-param, 13-21 second hash chain, 22 adjust-query, 23 adjust-fetch,
/// 24 adjust-apply, 25 attach, 26 register, 27 aim-hook (planted virtual),
/// 28 finish, 29 finish-id.
const OBJ_ID_OFF: u32 = 8;
const KIND_OFF: u32 = 0x1304;
const FLAG_BIT_OFF: u32 = 0x28;
const FLOAT0_OFF: u32 = 0x18;
const FLOAT1_OFF: u32 = 0x1c;
const FLOAT3_OFF: u32 = 0x24;
const FLOAT4_OFF: u32 = 0x20;
const IDX_OFF: u32 = 0x2e;
const TABLE_GLB: u32 = 0x1295cd8;
const ROW_INNER_OFF: u32 = 0xcc;
const WORD_A_OFF: u32 = 0xb4;
const WORD_B0_OFF: u32 = 0xb8;
const WORD_B1_OFF: u32 = 0xbc;
const FX_OBJ: u32 = 0x1394d60;
const AIM_SLOT: u32 = 0xec;
const AIM0_OFF: u32 = 0x190;
const AIM1_OFF: u32 = 0x194;
const AIM2_OFF: u32 = 0x198;
const FLAGSTORE_OFF: u32 = 0x1e2;
const FINISH_OFF: u32 = 0x1d4;
const CMP_GLB: u32 = 0x11f702c;
const VAL_GLB: u32 = 0x11f70c4;
const TRUCK_KIND: u32 = 4;

fn body<const MUT: bool>(this: u32, arg0: u32, arg1: u32) -> u32 {
    unsafe {
        if arg0 == 0 {
            return 0;
        }
        let fx = relocated(FX_OBJ);
        let obj_id = *(this.wrapping_add(OBJ_ID_OFF) as *const u32);
        let bit = u32::from(*(this.wrapping_add(FLAG_BIT_OFF) as *const u8) & 1);
        let read_idx = || *(arg0.wrapping_add(IDX_OFF) as *const i16) as i32;
        let read_word = |off: u32| {
            let row = *global::<u32>(TABLE_GLB).offset(read_idx() as isize);
            let inner = *(row.wrapping_add(ROW_INNER_OFF) as *const u32);
            *(inner.wrapping_add(off) as *const i32)
        };
        // First hash chain: the first two names select path A, the next
        // seven path B, otherwise the mode is -1. Calls stop at the first
        // match, exactly like the original.
        let h: u32 = callee_cdecl!(1, u32, relocated(0xebc070), 0);
        let path_a = if obj_id == h {
            true
        } else {
            let h: u32 = callee_cdecl!(2, u32, relocated(0xebc08c), 0);
            obj_id == h
        };
        let mut origin: u32 = 0;
        let mode: i32;
        if path_a {
            mode = 0;
            let v = read_word(WORD_A_OFF);
            if v > -1 {
                origin = callee_thiscall!(10, u32, arg0, v as u32);
            }
        } else {
            let h: u32 = callee_cdecl!(3, u32, relocated(0xebc0a4), 0);
            let mut hit = obj_id == h;
            if !hit {
                let h: u32 = callee_cdecl!(4, u32, relocated(0xebc0bc), 0);
                hit = obj_id == h;
            }
            if !hit {
                let h: u32 = callee_cdecl!(5, u32, relocated(0xebc0d4), 0);
                hit = obj_id == h;
            }
            if !hit {
                let h: u32 = callee_cdecl!(6, u32, relocated(0xebc0ec), 0);
                hit = obj_id == h;
            }
            if !hit {
                let h: u32 = callee_cdecl!(7, u32, relocated(0xebc104), 0);
                hit = obj_id == h;
            }
            if !hit {
                let h: u32 = callee_cdecl!(8, u32, relocated(0xebc11c), 0);
                hit = obj_id == h;
            }
            if !hit {
                let h: u32 = callee_cdecl!(9, u32, relocated(0xebc134), 0);
                hit = obj_id == h;
            }
            if hit {
                let off = if bit == 0 { WORD_B0_OFF } else { WORD_B1_OFF };
                let v = read_word(off);
                if v > -1 {
                    origin = callee_thiscall!(10, u32, arg0, v as u32);
                }
                mode = bit.wrapping_add(1) as i32;
            } else {
                mode = -1;
            }
        }
        // Create the emitter. The flag word is written by the callee stub;
        // its low byte gates the adjust path and the finish below.
        let mut flag: [u32; 1] = [0];
        let emitter: u32 = callee_thiscall!(
            11, u32, fx,
            arg0.wrapping_add(mode as u32),
            obj_id,
            flag.as_mut_ptr() as u32,
            0,
            0
        );
        if emitter == 0 {
            return 0;
        }
        let mode_f = f32::from_bits(arg1);
        let take_param = if MUT { mode_f != 0.0 } else { mode_f == 0.0 };
        if take_param {
            callee_thiscall!(
                12, u32, emitter,
                relocated(0xebc14c),
                *(this.wrapping_add(FLOAT0_OFF) as *const u32)
            );
            callee_thiscall!(
                12, u32, emitter,
                relocated(0xebc154),
                *(this.wrapping_add(FLOAT1_OFF) as *const u32)
            );
            callee_thiscall!(
                12, u32, emitter,
                relocated(0xebc15c),
                *(this.wrapping_add(FLOAT3_OFF) as *const u32)
            );
            // Second hash chain: two names take the early fourth set, three
            // the middle set, four the kind-gated set, otherwise no fourth
            // set. The early and empty variants skip the flag store below.
            let h: u32 = callee_cdecl!(13, u32, relocated(0xebc164), 0);
            let mut early = obj_id == h;
            if !early {
                let h: u32 = callee_cdecl!(14, u32, relocated(0xebc180), 0);
                early = obj_id == h;
            }
            if early {
                callee_thiscall!(
                    12, u32, emitter,
                    relocated(0xebc198),
                    *(this.wrapping_add(FLOAT4_OFF) as *const u32)
                );
            } else {
                let h: u32 = callee_cdecl!(15, u32, relocated(0xebc1a0), 0);
                let mut mid = obj_id == h;
                if !mid {
                    let h: u32 = callee_cdecl!(16, u32, relocated(0xebc1b8), 0);
                    mid = obj_id == h;
                }
                if !mid {
                    let h: u32 = callee_cdecl!(17, u32, relocated(0xebc1d0), 0);
                    mid = obj_id == h;
                }
                if mid {
                    callee_thiscall!(
                        12, u32, emitter,
                        relocated(0xebc1e8),
                        *(this.wrapping_add(FLOAT4_OFF) as *const u32)
                    );
                    if bit != 0 {
                        *(emitter.wrapping_add(FLAGSTORE_OFF) as *mut u8) = 1;
                    }
                } else {
                    let h: u32 = callee_cdecl!(18, u32, relocated(0xebc1f0), 0);
                    let mut late = obj_id == h;
                    if !late {
                        let h: u32 =
                            callee_cdecl!(19, u32, relocated(0xebc208), 0);
                        late = obj_id == h;
                    }
                    if !late {
                        let h: u32 =
                            callee_cdecl!(20, u32, relocated(0xebc220), 0);
                        late = obj_id == h;
                    }
                    if !late {
                        let h: u32 =
                            callee_cdecl!(21, u32, relocated(0xebc238), 0);
                        late = obj_id == h;
                    }
                    if late {
                        if *(arg0.wrapping_add(KIND_OFF) as *const u32)
                            == TRUCK_KIND
                        {
                            callee_thiscall!(
                                12, u32, emitter,
                                relocated(0xebc250),
                                *(this.wrapping_add(FLOAT4_OFF) as *const u32)
                            );
                        }
                        if bit != 0 {
                            *(emitter.wrapping_add(FLAGSTORE_OFF) as *mut u8) =
                                1;
                        }
                    }
                }
            }
        } else {
            // Adjust path: flag-gated, then a query, then fetch and apply.
            // The apply call's first word reads a frame slot the original
            // never stores to; the pinned zero fill is passed literally.
            if flag[0] & 0xFF == 0 {
                let go: u32 = callee_thiscall!(22, u32, this);
                if go & 0xFF != 0 {
                    let mut fetched: [u32; 1] = [0];
                    callee_cdecl!(
                        23, u32, fetched.as_mut_ptr() as u32, this
                    );
                    callee_thiscall!(24, u32, this, 0, arg0, fetched[0], arg1);
                }
            }
        }
        callee_thiscall!(25, u32, emitter, origin);
        callee_thiscall!(26, u32, fx, emitter, arg0, 0);
        // The record's aim hook, called through its vtable exactly like the
        // original; both sides land on the same planted stub.
        let vtable = *(arg0 as *const u32);
        let hook: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vtable.wrapping_add(AIM_SLOT))
                as *const u32));
        let mut scratch: [u32; 1] = [0];
        let ans = hook(arg0, scratch.as_mut_ptr() as u32);
        *(emitter.wrapping_add(AIM0_OFF) as *mut u32) =
            *(ans as *const u32);
        *(emitter.wrapping_add(AIM1_OFF) as *mut u32) =
            *((ans.wrapping_add(4)) as *const u32);
        *(emitter.wrapping_add(AIM2_OFF) as *mut u32) =
            *((ans.wrapping_add(8)) as *const u32);
        if flag[0] & 0xFF != 0 {
            callee_thiscall!(28, u32, emitter);
            let idans: u32 = callee_cdecl!(29, u32,);
            let mut e = *global::<u32>(VAL_GLB);
            if *global::<u32>(CMP_GLB) == idans {
                e = e.wrapping_add(1);
            }
            *(emitter.wrapping_add(FINISH_OFF) as *mut u32) = e;
        }
        0
    }
}

export!(thiscall, rw_00bf5190(this: u32, arg0: u32, arg1: u32) -> u32 {
    body::<false>(this, arg0, arg1)
});

export!(thiscall, mut_00bf5190(this: u32, arg0: u32, arg1: u32) -> u32 {
    body::<true>(this, arg0, arg1)
});
