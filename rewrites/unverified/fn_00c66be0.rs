// original: 0x00C66BE0 CCutsceneObject::vf33
/// Updates the object's selected 3×3 transform from two scripted double
/// values, after running its optional preparation hooks and sentinel list.
/// The signed 16-bit key selects the metadata entry. A zero mode byte also
/// clears the transform's translation column. The method returns the update
/// result when available, otherwise it asks the object's fallback hook.
#[inline(never)]
unsafe fn rewrite_body(this: u32) -> u32 {
    const NESTED_OBJECT: u32 = 0x290;
    const LINK_OBJECT: u32 = 0x34;
    const LOOP_VALUES: u32 = 0x2b8;
    const KEY: u32 = 0x2e;
    const MATRIX_MODE: u32 = 0x2ab;
    const FALLBACK_OBJECT: u32 = 0x100;
    const PTR_TABLE_FILE_VA: u32 = 0x0129_5cd8;
    const POINTER_FIELD: u32 = 0xcc;
    const READY_FIELD: u32 = 0x18c;

    const GATE_A: u32 = 1;
    const GATE_B: u32 = 2;
    const NESTED_HOOK: u32 = 3;
    const LINK_HOOK: u32 = 4;
    const HANDLE_HOOK: u32 = 5;
    const HANDLE_RESOLVE: u32 = 6;
    const LIST_VISIT: u32 = 7;
    const TABLE_ACTION: u32 = 8;
    const HANDLE_INDEX: u32 = 9;
    const MATRIX_SELECT: u32 = 10;
    const COSINE_SOURCE: u32 = 11;
    const SINE_SOURCE: u32 = 12;
    const MATRIX_FINISH: u32 = 13;
    const FALLBACK_RESULT: u32 = 14;

    #[inline(always)]
    unsafe fn read_u32(address: u32) -> u32 {
        unsafe { (address as *const u32).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn write_u32(address: u32, value: u32) {
        unsafe { (address as *mut u32).write_unaligned(value) }
    }

    #[inline(always)]
    unsafe fn read_u16(address: u32) -> u16 {
        unsafe { (address as *const u16).read_unaligned() }
    }

    #[inline(always)]
    unsafe fn read_u8(address: u32) -> u8 {
        unsafe { (address as *const u8).read() }
    }

    #[inline(always)]
    unsafe fn read_f32(address: u32) -> f32 {
        f32::from_bits(unsafe { read_u32(address) })
    }

    #[inline(always)]
    unsafe fn write_f32(address: u32, value: f32) {
        unsafe { write_u32(address, value.to_bits()) }
    }

    #[inline(never)]
    fn multiply(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        core::hint::black_box(left * right)
    }

    #[inline(never)]
    fn add(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        core::hint::black_box(left + right)
    }

    #[inline(never)]
    fn subtract(left: f32, right: f32) -> f32 {
        let left = core::hint::black_box(left);
        let right = core::hint::black_box(right);
        core::hint::black_box(left - right)
    }

    #[inline(never)]
    fn double_to_float(bits: u64) -> f32 {
        let value = core::hint::black_box(f64::from_bits(bits));
        core::hint::black_box(value as f32)
    }

    let gate_a = unsafe {
        lf_checker_rt::callee_thiscall!(
            GATE_A,
            u32,
            this,
        )
    };
    if gate_a as u8 != 0 {
        let nested = unsafe { read_u32(this.wrapping_add(NESTED_OBJECT)) };
        let _ = unsafe { lf_checker_rt::callee_thiscall!(NESTED_HOOK, u32, nested) };
    }

    let gate_b = unsafe {
        lf_checker_rt::callee_thiscall!(
            GATE_B,
            u32,
            this,
        )
    };
    if gate_b as u8 != 0 {
        let link = unsafe { read_u32(this.wrapping_add(LINK_OBJECT)) };
        let nested = unsafe { read_u32(link.wrapping_add(4)) };
        let _ = unsafe { lf_checker_rt::callee_thiscall!(LINK_HOOK, u32, nested, this) };

        for slot in 0..10u32 {
            let value = unsafe { read_u32(this.wrapping_add(LOOP_VALUES + slot * 4)) };
            if value != u32::MAX {
                let _ = unsafe { lf_checker_rt::callee_thiscall!(LIST_VISIT, u32, this, value) };
            }
        }

        let key = unsafe { read_u16(this.wrapping_add(KEY)) as i16 as i32 };
        let table_base = lf_checker_rt::global::<u32>(PTR_TABLE_FILE_VA) as u32;
        let table_slot = table_base.wrapping_add((key.wrapping_mul(4)) as u32);
        let selected = unsafe { read_u32(table_slot) };
        let selected_info = unsafe { read_u32(selected.wrapping_add(POINTER_FIELD)) };
        if unsafe { read_u32(selected_info.wrapping_add(READY_FIELD)) } != 0 {
            let _ = unsafe {
                lf_checker_rt::callee_thiscall!(TABLE_ACTION, u32, this, 10u32, selected)
            };
            let _ = unsafe {
                lf_checker_rt::callee_thiscall!(TABLE_ACTION, u32, this, 12u32, selected)
            };
            let _ = unsafe {
                lf_checker_rt::callee_thiscall!(TABLE_ACTION, u32, this, 14u32, selected)
            };
        }
    }

    let handle = unsafe { lf_checker_rt::callee_thiscall!(HANDLE_HOOK, u32, this) };
    let handle = if handle != 0 {
        let resolved = unsafe { lf_checker_rt::callee_thiscall!(HANDLE_HOOK, u32, this) };
        unsafe { lf_checker_rt::callee_thiscall!(HANDLE_RESOLVE, u32, resolved) }
    } else {
        unsafe { read_u32(this.wrapping_add(FALLBACK_OBJECT)) }
    };
    let handle_value = unsafe { read_u32(handle.wrapping_add(4)) };
    let lookup = unsafe { lf_checker_rt::callee_cdecl!(HANDLE_INDEX, u32, handle_value, 0u32) };
    let lookup = if lookup == u32::MAX { 0 } else { lookup };
    let matrix = unsafe { lf_checker_rt::callee_thiscall!(MATRIX_SELECT, u32, this, lookup) };

    if unsafe { read_u8(this.wrapping_add(MATRIX_MODE)) } == 0 {
        unsafe {
            write_u32(matrix.wrapping_add(0x30), 0);
            write_u32(matrix.wrapping_add(0x34), 0);
            write_u32(matrix.wrapping_add(0x38), 0);
            // The source value is an uninitialized stack word; the contract fixes it to zero.
            write_u32(matrix.wrapping_add(0x3c), 0);
        }
    }

    unsafe {
        write_u32(matrix, 0x3f80_0000);
        write_u32(matrix.wrapping_add(4), 0);
        write_u32(matrix.wrapping_add(8), 0);
        write_u32(matrix.wrapping_add(0x10), 0);
        write_u32(matrix.wrapping_add(0x14), 0x3f80_0000);
        write_u32(matrix.wrapping_add(0x18), 0);
        write_u32(matrix.wrapping_add(0x20), 0);
        write_u32(matrix.wrapping_add(0x24), 0);
        write_u32(matrix.wrapping_add(0x28), 0x3f80_0000);
    }

    let cosine_bits = unsafe { lf_checker_rt::callee_cdecl!(COSINE_SOURCE, u64, 0u32, 0u32) };
    let cosine = double_to_float(cosine_bits);
    let sine_bits = unsafe { lf_checker_rt::callee_cdecl!(SINE_SOURCE, u64, 0u32, 0u32) };
    let sine = double_to_float(sine_bits);

    let row1 = unsafe {
        [
            read_f32(matrix.wrapping_add(0x10)),
            read_f32(matrix.wrapping_add(0x14)),
            read_f32(matrix.wrapping_add(0x18)),
        ]
    };
    let row1_cos = [
        multiply(row1[0], cosine),
        multiply(row1[1], cosine),
        multiply(row1[2], cosine),
    ];
    let row2_after_first = [
        subtract(0.0, multiply(row1[0], sine)),
        subtract(0.0, multiply(row1[1], sine)),
        subtract(cosine, multiply(row1[2], sine)),
    ];
    let row1_after_first = [
        add(0.0, row1_cos[0]),
        add(0.0, row1_cos[1]),
        add(row1_cos[2], sine),
    ];
    unsafe {
        write_f32(matrix.wrapping_add(0x10), row1_after_first[0]);
        write_f32(matrix.wrapping_add(0x14), row1_after_first[1]);
        write_f32(matrix.wrapping_add(0x18), row1_after_first[2]);
        write_f32(matrix.wrapping_add(0x20), row2_after_first[0]);
        write_f32(matrix.wrapping_add(0x24), row2_after_first[1]);
        write_f32(matrix.wrapping_add(0x28), row2_after_first[2]);
    }

    let row0_before_second = unsafe {
        [
            read_f32(matrix),
            read_f32(matrix.wrapping_add(4)),
            read_f32(matrix.wrapping_add(8)),
        ]
    };
    let row2_before_second = unsafe {
        [
            read_f32(matrix.wrapping_add(0x20)),
            read_f32(matrix.wrapping_add(0x24)),
            read_f32(matrix.wrapping_add(0x28)),
        ]
    };
    let row0_after_second = [
        subtract(
            multiply(row0_before_second[0], cosine),
            multiply(sine, row2_before_second[0]),
        ),
        subtract(
            multiply(row0_before_second[1], cosine),
            multiply(sine, row2_before_second[1]),
        ),
        subtract(
            multiply(row0_before_second[2], cosine),
            multiply(sine, row2_before_second[2]),
        ),
    ];
    let row2_scaled = [
        multiply(row2_before_second[0], cosine),
        multiply(row2_before_second[1], cosine),
        multiply(row2_before_second[2], cosine),
    ];
    let row2_after_second = [
        add(multiply(row0_before_second[0], sine), row2_scaled[0]),
        add(multiply(row0_before_second[1], sine), row2_scaled[1]),
        add(multiply(row0_before_second[2], sine), row2_scaled[2]),
    ];
    unsafe {
        write_f32(matrix, row0_after_second[0]);
        write_f32(matrix.wrapping_add(4), row0_after_second[1]);
        write_f32(matrix.wrapping_add(8), row0_after_second[2]);
        write_f32(matrix.wrapping_add(0x20), row2_after_second[0]);
        write_f32(matrix.wrapping_add(0x24), row2_after_second[1]);
        write_f32(matrix.wrapping_add(0x28), row2_after_second[2]);
    }

    let row0_before_third = row0_after_second;
    let row1_before_third = row1_after_first;
    let row2_before_third = row2_after_second;
    let row0_after_third = [
        add(multiply(row1_before_third[0], sine), row2_before_third[0]),
        add(
            multiply(row1_before_third[1], sine),
            multiply(row0_before_third[1], cosine),
        ),
        add(
            multiply(row1_before_third[2], sine),
            multiply(row0_before_third[2], cosine),
        ),
    ];
    let row1_after_third = [
        subtract(
            multiply(row1_before_third[0], cosine),
            multiply(row0_before_third[0], sine),
        ),
        subtract(
            multiply(row1_before_third[1], cosine),
            multiply(row0_before_third[1], sine),
        ),
        subtract(
            multiply(row1_before_third[2], cosine),
            multiply(row0_before_third[2], sine),
        ),
    ];
    unsafe {
        write_f32(matrix, row0_after_third[0]);
        write_f32(matrix.wrapping_add(4), row0_after_third[1]);
        write_f32(matrix.wrapping_add(8), row0_after_third[2]);
        write_f32(matrix.wrapping_add(0x10), row1_after_third[0]);
        write_f32(matrix.wrapping_add(0x14), row1_after_third[1]);
        write_f32(matrix.wrapping_add(0x18), row1_after_third[2]);
    }

    let result = unsafe { lf_checker_rt::callee_thiscall!(MATRIX_FINISH, u32, matrix) };
    if result == 0 {
        let _ = unsafe { lf_checker_rt::callee_thiscall!(FALLBACK_RESULT, u32, this) };
    }
    result
}

lf_checker_rt::export!(thiscall, rw_00C66BE0(this: u32) -> u32 {
    unsafe { rewrite_body(this) }
});
