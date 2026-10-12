// original: 0x00bf9120 fx_heli_update

/// Update a helicopter effect slot from the task and its request object.
///
/// The routine first checks whether the request is already covered by a
/// nearby stream slot. If not, it acquires a slot, builds the request's
/// transform data, and selects either the matrix attachment path or the
/// direct transform path from the task flags. Those flags also choose the
/// effect parameters and optional sample bake. A qualifying request type
/// selects three normalized channels from its color-table entry.
/// Finally the slot receives its update stamp and the task's slot-table
/// allocation result is returned.
///
/// The scratch frame preserves the original's shared temporary buffers across
/// calls. Every intercepted call remains visible to the contract, including
/// pointer snapshots for those temporary buffers.
lf_checker_rt::export!(thiscall, rw_00bf9120(this: u32, request: u32) -> u32 {
    unsafe {
        const SLOT_MANAGER: u32 = 0x01394D60;
        const COLOR_SCALE: u32 = 0x00FE86E8;
        const COLOR_TABLE: u32 = 0x012FAE88;
        const PARAM_KEY: u32 = 0x00EBC724;
        const ENABLED_KEY: u32 = 0x00EBC72C;
        const DISABLED_KEY: u32 = 0x00EBC734;
        const TICK_COMPARE: u32 = 0x011F702C;
        const TICK_VALUE: u32 = 0x011F70C4;

        const C_COPY_THREE: u32 = 1;
        const C_SLOT_IN_RADIUS: u32 = 2;
        const C_SLOT_DISPATCH: u32 = 3;
        const C_BYTE_VECTOR: u32 = 4;
        const C_RANDOM_BASIS: u32 = 5;
        const C_ALLOC_MATRIX: u32 = 6;
        const C_PLACE_MATRIX: u32 = 7;
        const C_FIRE_WEAPON: u32 = 8;
        const C_MATRIX_INIT: u32 = 9;
        const C_MATRIX_APPLY: u32 = 10;
        const C_ATTACH_TRANSFORM: u32 = 11;
        const C_SET_MATRIX: u32 = 12;
        const C_SLOT_ADOPT: u32 = 13;
        const C_SET_TRANSFORM: u32 = 14;
        const C_SET_FLOAT: u32 = 15;
        const C_FREE_SAMPLES: u32 = 16;
        const C_RAYCAST_BAKE: u32 = 17;
        const C_CAPTURE_BAKE: u32 = 18;
        const C_SLOT_UPDATE: u32 = 19;
        const C_TICK_CHECK: u32 = 20;
        const C_ALLOC_SLOT: u32 = 21;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let mut scratch = [0u32; 58];
        let frame = scratch.as_mut_ptr() as u32;
        write_u32(frame.wrapping_add(0x0C), this);

        lf_checker_rt::callee_thiscall!(
            C_COPY_THREE,
            u32,
            this,
            frame.wrapping_add(0x10)
        );

        let near_answer = lf_checker_rt::callee_thiscall!(
            C_SLOT_IN_RADIUS,
            u32,
            lf_checker_rt::relocated(SLOT_MANAGER),
            read_u32(this.wrapping_add(0x08)),
            frame.wrapping_add(0x10),
            0x3E80_0000
        );
        if near_answer & 0xFF != 0 {
            return near_answer;
        }

        let slot = lf_checker_rt::callee_thiscall!(
            C_SLOT_DISPATCH,
            u32,
            lf_checker_rt::relocated(SLOT_MANAGER),
            read_u32(this.wrapping_add(0x08)),
            0,
            0
        );
        if slot == 0 {
            return slot;
        }

        lf_checker_rt::callee_thiscall!(
            C_BYTE_VECTOR,
            u32,
            this,
            frame.wrapping_add(0x20)
        );
        lf_checker_rt::callee_cdecl!(
            C_RANDOM_BASIS,
            u32,
            frame.wrapping_add(0xB0),
            frame.wrapping_add(0x10),
            frame.wrapping_add(0x20),
            0
        );

        let task_flags = read_u8(this.wrapping_add(0x24));
        if task_flags & 0x04 != 0 {
            if read_u32(request.wrapping_add(0x20)) == 0 {
                lf_checker_rt::callee_thiscall!(C_ALLOC_MATRIX, u32, request);
                lf_checker_rt::callee_thiscall!(
                    C_PLACE_MATRIX,
                    u32,
                    request.wrapping_add(0x10),
                    read_u32(request.wrapping_add(0x20))
                );
            }

            lf_checker_rt::callee_thiscall!(
                C_FIRE_WEAPON,
                u32,
                frame.wrapping_add(0x70),
                read_u32(request.wrapping_add(0x20))
            );
            lf_checker_rt::callee_thiscall!(
                C_MATRIX_INIT,
                u32,
                frame.wrapping_add(0x70)
            );
            lf_checker_rt::callee_thiscall!(
                C_FIRE_WEAPON,
                u32,
                frame.wrapping_add(0x30),
                frame.wrapping_add(0xB0)
            );
            lf_checker_rt::callee_thiscall!(
                C_MATRIX_APPLY,
                u32,
                frame.wrapping_add(0x30),
                frame.wrapping_add(0x70)
            );

            if read_u32(request.wrapping_add(0x20)) == 0 {
                lf_checker_rt::callee_thiscall!(C_ALLOC_MATRIX, u32, request);
                lf_checker_rt::callee_thiscall!(
                    C_PLACE_MATRIX,
                    u32,
                    request.wrapping_add(0x10),
                    read_u32(request.wrapping_add(0x20))
                );
            }
            lf_checker_rt::callee_thiscall!(
                C_ATTACH_TRANSFORM,
                u32,
                slot,
                read_u32(request.wrapping_add(0x20))
            );
            lf_checker_rt::callee_thiscall!(
                C_SET_MATRIX,
                u32,
                slot,
                frame.wrapping_add(0x30)
            );
            lf_checker_rt::callee_thiscall!(
                C_SLOT_ADOPT,
                u32,
                lf_checker_rt::relocated(SLOT_MANAGER),
                slot,
                request,
                0
            );
        } else {
            lf_checker_rt::callee_thiscall!(
                C_SET_TRANSFORM,
                u32,
                slot,
                frame.wrapping_add(0xB0)
            );
        }

        if task_flags & 0x04 != 0 && request != 0 {
            let effect_type = read_u32(request.wrapping_add(0x28)) & 0x3C0;
            if effect_type == 0x80 {
                let color_index = u32::from(read_u8(request.wrapping_add(0xF94)));
                let packed = read_u32(
                    lf_checker_rt::relocated(COLOR_TABLE.wrapping_add(color_index.wrapping_mul(4)))
                );
                let scale = f32::from_bits(read_u32(lf_checker_rt::relocated(COLOR_SCALE)));
                let red = ((packed >> 16) & 0xFF) as f32
                    * core::hint::black_box(scale);
                let green = ((packed >> 8) & 0xFF) as f32
                    * core::hint::black_box(scale);
                let blue = (packed & 0xFF) as f32
                    * core::hint::black_box(scale);
                write_u32(slot.wrapping_add(0x180), red.to_bits());
                write_u32(slot.wrapping_add(0x184), green.to_bits());
                write_u32(slot.wrapping_add(0x188), blue.to_bits());
            }
        }

        lf_checker_rt::callee_thiscall!(
            C_SET_FLOAT,
            u32,
            slot,
            lf_checker_rt::relocated(PARAM_KEY),
            read_u32(this.wrapping_add(0x20))
        );
        let (enable_key, enable_value) = if task_flags & 0x01 != 0 {
            (ENABLED_KEY, 0x3F80_0000)
        } else {
            (DISABLED_KEY, 0)
        };
        lf_checker_rt::callee_thiscall!(
            C_SET_FLOAT,
            u32,
            slot,
            lf_checker_rt::relocated(enable_key),
            enable_value
        );

        match (task_flags >> 3) & 0x03 {
            1 => {
                lf_checker_rt::callee_thiscall!(C_FREE_SAMPLES, u32, slot);
                lf_checker_rt::callee_cdecl!(
                    C_RAYCAST_BAKE,
                    u32,
                    slot,
                    frame.wrapping_add(0x10),
                    0x4080_0000,
                    0x40C0_0000,
                    0xBF80_0000
                );
            }
            2 => {
                lf_checker_rt::callee_thiscall!(C_FREE_SAMPLES, u32, slot);
                lf_checker_rt::callee_cdecl!(
                    C_CAPTURE_BAKE,
                    u32,
                    slot,
                    frame.wrapping_add(0x10),
                    0x4020_0000,
                    0xBF80_0000
                );
            }
            _ => {}
        }

        if task_flags & 0x01 != 0 {
            write_u32(slot.wrapping_add(0x1F0), 0x4040_0000);
        } else if task_flags & 0x02 != 0 {
            write_u32(slot.wrapping_add(0x1F0), 0x4000_0000);
        }

        lf_checker_rt::callee_thiscall!(C_SLOT_UPDATE, u32, slot);
        let check = lf_checker_rt::callee_cdecl!(C_TICK_CHECK, u32,);
        let compare = read_u32(lf_checker_rt::relocated(TICK_COMPARE));
        let mut tick = read_u32(lf_checker_rt::relocated(TICK_VALUE));
        if check == compare {
            tick = tick.wrapping_add(1);
        }
        write_u32(slot.wrapping_add(0x1D4), tick);

        lf_checker_rt::callee_thiscall!(
            C_ALLOC_SLOT,
            u32,
            lf_checker_rt::relocated(SLOT_MANAGER),
            read_u32(this.wrapping_add(0x08)),
            frame.wrapping_add(0x10),
            0x3E4C_CCCD
        )
    }
});
