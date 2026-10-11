// original: 0x00DADEA0 CTaskComplexAvoidPlayerTargetting::vf20

/// Advances the avoid-player task after validating its two references and
/// age, then either updates a direction or returns the associated ped.
///
/// `this+0x14` and `this+0x18` hold the active references, `this+0x08` holds
/// the ped, and the incoming context points to the direction matrix at
/// `+0x20`. A successful preparation query sets the local prepared flag; the
/// task then checks the ped's virtual state, runs the 0x19f direction path or
/// the 0x11d coordinate-refresh path, and falls back to the ped's slot
/// `+0x14` method when its references or age are unsuitable. The direct
/// helpers stay calls, while the virtual queries load their planted targets
/// from the object's function tables.
lf_checker_rt::export!(thiscall, rw_00DADEA0(this: u32, context: u32) -> u32 {
    const PED: u32 = 0x08;
    const FIRST_REFERENCE: u32 = 0x14;
    const SECOND_REFERENCE: u32 = 0x18;
    const PREPARED_FLAG: u32 = 0x1C;
    const RECENT_ANGLE_FLAG: u32 = 0x30;
    const LAST_UPDATE: u32 = 0x34;
    const REFERENCE_STATE: u32 = 0x398;
    const REFERENCE_AUX: u32 = 0x2C4;
    const REFERENCE_MANAGER: u32 = 0x228;
    const MANAGER_OFFSET: u32 = 0x70;
    const MANAGER_READY: u32 = 0x3D0;
    const TIMER_VA: u32 = 0x0117_35B4;
    const TIMER_LIMIT: u32 = 1000;
    const PED_STATE: u32 = 0x0C;
    const PED_FALLBACK_SLOT: u32 = 0x14;
    const FRAME_MATRIX: u32 = 0x20;
    const MATRIX_X: u32 = 0x00;
    const MATRIX_Y: u32 = 0x04;
    const MATRIX_RIGHT_X: u32 = 0x10;
    const MATRIX_RIGHT_Y: u32 = 0x14;
    const PED_ANGLE_X: u32 = 0x20;
    const PED_ANGLE_Y: u32 = 0x24;
    const PED_ANGLE_Z: u32 = 0x28;
    const EXPECTED_ANGLE_Y_VA: u32 = 0x00ED_7E68;
    const EXPECTED_ANGLE_Z_VA: u32 = 0x00ED_7E70;
    const PREPARE_HELPER: u32 = 1;
    const PREPARE_OUTPUT_HELPER: u32 = 2;
    const SETUP_PED_HELPER: u32 = 3;
    const ELIGIBILITY_HELPER: u32 = 4;
    const BUILD_DIRECTIONS_HELPER: u32 = 5;
    const UPDATE_HEADING_HELPER: u32 = 9;
    const FIND_PED_HELPER: u32 = 10;
    const COMPUTE_ANGLE_HELPER: u32 = 11;
    const VTABLE_QUERY: u32 = 0x0C;
    const VTABLE_FALLBACK: u32 = PED_FALLBACK_SLOT;

    #[inline(always)]
    unsafe fn read_word(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_byte(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    #[inline(always)]
    unsafe fn write_word(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn read_float(address: u32) -> f32 {
        f32::from_bits(unsafe { read_word(address) })
    }

    #[inline(always)]
    fn add(left: f32, right: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(left) + core::hint::black_box(right))
    }

    #[inline(always)]
    fn multiply(left: f32, right: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(left) * core::hint::black_box(right))
    }

    #[inline(always)]
    fn divide(left: f32, right: f32) -> f32 {
        core::hint::black_box(core::hint::black_box(left) / core::hint::black_box(right))
    }

    #[inline(always)]
    unsafe fn virtual_query(object: u32, slot: u32) -> u32 {
        let table = unsafe { read_word(object) };
        let target = unsafe { read_word(table.wrapping_add(slot)) };
        let function: extern "thiscall" fn(u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        function(object)
    }

    #[inline(always)]
    unsafe fn virtual_fallback(object: u32, slot: u32, context: u32) -> u32 {
        let table = unsafe { read_word(object) };
        let target = unsafe { read_word(table.wrapping_add(slot)) };
        let function: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            unsafe { core::mem::transmute(target as usize) };
        function(object, context, 1, 0)
    }

    #[inline(always)]
    unsafe fn fallback(this: u32, ped: u32, context: u32) -> u32 {
        let flags = unsafe { read_word(ped.wrapping_add(PED_STATE)) };
        if flags & 1 != 0 {
            return 0;
        }
        let result = unsafe { virtual_fallback(ped, VTABLE_FALLBACK, context) };
        if result & 0xFF != 0 {
            unsafe { write_word(ped.wrapping_add(PED_STATE), flags | 2) };
            0
        } else {
            let _ = this;
            ped
        }
    }

    unsafe {
        let mut preparation_output = [0u32; 8];
        let mut direction_output = [0u32; 8];
        direction_output[0] = 1;
        let mut prepared = false;
        let first_reference = read_word(this.wrapping_add(FIRST_REFERENCE));
        if first_reference != 0
            && read_word(first_reference.wrapping_add(REFERENCE_STATE)) == 0
        {
            let auxiliary = read_word(first_reference.wrapping_add(REFERENCE_AUX));
            let manager = lf_checker_rt::callee_thiscall!(
                PREPARE_HELPER,
                u32,
                first_reference.wrapping_add(0x2B0)
            );
            if manager != 0 && auxiliary != 0 {
                let state = read_word(first_reference.wrapping_add(REFERENCE_MANAGER));
                let ready_at = if state == 0 {
                    0x3D0
                } else {
                    state.wrapping_add(MANAGER_OFFSET).wrapping_add(MANAGER_READY)
                };
                if read_byte(ready_at) & 8 != 0 {
                    let first_out = preparation_output.as_mut_ptr() as u32;
                    let second_out = first_out.wrapping_add(16);
                    let _ = lf_checker_rt::callee_fastcall!(
                        PREPARE_OUTPUT_HELPER,
                        u32,
                        manager,
                        manager,
                        context,
                        read_word(auxiliary.wrapping_add(0x20)),
                        first_out,
                        second_out
                    );
                    prepared = true;
                }
            }
        }

        let first_reference = read_word(this.wrapping_add(FIRST_REFERENCE));
        let second_reference = read_word(this.wrapping_add(SECOND_REFERENCE));
        if first_reference == 0 || second_reference == 0 {
            return fallback(this, read_word(this.wrapping_add(PED)), context);
        }
        let reference_state = read_word(first_reference.wrapping_add(REFERENCE_STATE));
        if reference_state != 0 && reference_state != second_reference {
            return fallback(this, read_word(this.wrapping_add(PED)), context);
        }
        if !prepared && reference_state == 0 {
            return fallback(this, read_word(this.wrapping_add(PED)), context);
        }
        let elapsed = lf_checker_rt::global::<u32>(TIMER_VA)
            .read_unaligned()
            .wrapping_sub(read_word(this.wrapping_add(LAST_UPDATE)));
        if elapsed > TIMER_LIMIT {
            return fallback(this, read_word(this.wrapping_add(PED)), context);
        }

        let ped = read_word(this.wrapping_add(PED));
        let ped_kind = virtual_query(ped, VTABLE_QUERY);
        if ped_kind == 0x11D {
            let related = read_word(ped.wrapping_add(SECOND_REFERENCE));
            if related == 0 || virtual_query(related, VTABLE_QUERY) != 0x386 {
                return ped;
            }
            let target = lf_checker_rt::callee_thiscall!(FIND_PED_HELPER, u32, ped, context);
            if target == 0 {
                return ped;
            }
            let angle = lf_checker_rt::callee_thiscall!(
                COMPUTE_ANGLE_HELPER,
                f32,
                this,
                context
            );
            let target_x = read_float(target.wrapping_add(PED_ANGLE_X));
            let target_y = read_float(target.wrapping_add(PED_ANGLE_Y));
            let target_z = read_float(target.wrapping_add(PED_ANGLE_Z));
            let expected_y = lf_checker_rt::global::<f32>(EXPECTED_ANGLE_Y_VA).read_unaligned();
            let expected_z = lf_checker_rt::global::<f32>(EXPECTED_ANGLE_Z_VA).read_unaligned();
            if target_x != angle || target_y != expected_y || target_z != expected_z {
                write_word(target.wrapping_add(PED_ANGLE_X), angle.to_bits());
                write_word(target.wrapping_add(PED_ANGLE_Y), expected_y.to_bits());
                write_word(target.wrapping_add(PED_ANGLE_Z), expected_z.to_bits());
            }
            return ped;
        }
        if ped_kind != 0x19F {
            return ped;
        }

        let _ = lf_checker_rt::callee_thiscall!(SETUP_PED_HELPER, u32, ped, 0x64);
        let allow_direction = if read_byte(this.wrapping_add(PREPARED_FLAG)) != 0 {
            lf_checker_rt::callee_cdecl!(
                ELIGIBILITY_HELPER,
                u32,
                first_reference,
                second_reference,
                this.wrapping_add(0x20),
                0x3F00_0000
            ) & 0xFF
                == 0
        } else {
            true
        };

        if read_byte(this.wrapping_add(RECENT_ANGLE_FLAG)) != 0 || !allow_direction {
            let _ = lf_checker_rt::callee_thiscall!(
                UPDATE_HEADING_HELPER,
                u32,
                ped,
                0,
                0,
                0
            );
            return ped;
        }

        let first_out = direction_output.as_mut_ptr() as u32;
        let second_out = first_out.wrapping_add(16);
        let _ = lf_checker_rt::callee_thiscall!(
            BUILD_DIRECTIONS_HELPER,
            u32,
            this,
            first_out,
            second_out
        );
        let matrix = read_word(context.wrapping_add(FRAME_MATRIX));
        let second_x = f32::from_bits(direction_output[4]);
        let second_y = f32::from_bits(direction_output[5]);
        let dot_right = add(
            multiply(read_float(matrix.wrapping_add(MATRIX_RIGHT_Y)), second_y),
            multiply(read_float(matrix.wrapping_add(MATRIX_RIGHT_X)), second_x),
        );
        let dot_forward = add(
            multiply(read_float(matrix.wrapping_add(MATRIX_Y)), second_y),
            multiply(read_float(matrix.wrapping_add(MATRIX_X)), second_x),
        );
        let abs_mask = lf_checker_rt::global::<u32>(0x00FE_8F80).read_unaligned();
        let right_magnitude = f32::from_bits(dot_right.to_bits() & abs_mask);
        let forward_magnitude = f32::from_bits(dot_forward.to_bits() & abs_mask);
        let (heading_x, heading_y) = if forward_magnitude > right_magnitude {
            (divide(dot_forward, forward_magnitude), 0.0)
        } else {
            let sign_mask = lf_checker_rt::global::<u32>(0x00FE_8FA0).read_unaligned();
            (
                0.0,
                f32::from_bits(divide(dot_right, right_magnitude).to_bits() ^ sign_mask),
            )
        };
        let _ = lf_checker_rt::callee_thiscall!(
            UPDATE_HEADING_HELPER,
            u32,
            ped,
            heading_x.to_bits(),
            heading_y.to_bits(),
            0
        );
        read_word(this.wrapping_add(PED))
    }
});
