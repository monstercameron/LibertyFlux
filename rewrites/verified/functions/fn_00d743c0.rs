// original: 0x00D743C0 overlay_dual_emit
/// Initializes the overlay's render values, selects its quality mode, and
/// emits both table-selected passes for each of seventeen paired records.
/// Returns the final service result after forwarding the state and draw calls.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

const D_C_FETCH_MATRIX: u32 = 1;
const D_C_SUBMIT_BASE: u32 = 2;
const D_C_SET_PAIR: u32 = 3;
const D_C_LOOKUP_SELECTOR: u32 = 4;
const D_C_SET_SELECTED: u32 = 5;
const D_C_SET_QUALITY: u32 = 6;
const D_C_SET_PASS: u32 = 7;
const D_C_SET_SCALE: u32 = 8;
const D_C_FETCH_ELEMENTS: u32 = 9;
const D_C_DRAW_MODE: u32 = 10;
const D_C_APPLY_EMISSION: u32 = 11;
const D_C_RESET_STATE: u32 = 12;
const D_C_GET_TEXTURE: u32 = 13;
const D_C_EMIT_ENTRY: u32 = 14;
const D_C_FINISH: u32 = 15;

const D_G_BASE_A: u32 = 0x0118EFFC;
const D_G_BASE_B: u32 = 0x0118EFF4;
const D_G_ADJUST_A: u32 = 0x00FE87E8;
const D_G_ADJUST_B: u32 = 0x00FE86EC;
const D_G_MODE_A: u32 = 0x0116C250;
const D_G_MODE_B: u32 = 0x0116C253;
const D_G_TEXTURE_TABLE_A: u32 = 0x01056A70;
const D_G_TEXTURE_TABLE_B: u32 = 0x01056AB8;
const D_G_TEXTURE_ECX: u32 = 0x0116BFF0;

const D_MATRIX_INDEX: u32 = 0x2C;
const D_LOOKUP_KEY: u32 = 0x3B;
const D_SPECIAL_MODE_BYTE: u8 = 0x6A;
const D_QUALITY_SPECIAL: u32 = 7;
const D_QUALITY_NORMAL: u32 = 2;
const D_PASS_FIRST: u32 = 1;
const D_PASS_SECOND: u32 = 2;
const D_SCALE_BITS: u32 = 0x3F8C_CCCD;
const D_PALETTE_ENTRY: u32 = 2;
const D_PALETTE_ALPHA_BYTE: u32 = 0xC8;
const D_DRAW_SOURCE: u32 = 0;
const D_DRAW_FLAGS: u32 = 1;
const D_RENDER_COMPONENT_OFFSET: u32 = 0xEC;
const D_RENDER_RECORDS_OFFSET: u32 = 0x15C;
const D_TEXTURE_ENTRY_SIZE: u32 = 4;
const D_RENDER_RECORD_SIZE: u32 = 8;
const D_RENDER_RECORD_COUNT: u32 = 17;

#[inline(always)]
unsafe fn global_f32(address: u32) -> f32 {
    f32::from_bits(unsafe { global::<u32>(address).read() })
}

#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 {
    f32::from_bits(unsafe { read_u32(address) })
}

#[inline(always)]
fn add_f32_ordered(left: f32, right: f32) -> f32 {
    let result = core::hint::black_box(left) + core::hint::black_box(right);
    core::hint::black_box(left);
    core::hint::black_box(right);
    result
}

#[inline(always)]
fn sub_f32_ordered(left: f32, right: f32) -> f32 {
    core::hint::black_box(left);
    core::hint::black_box(right);
    let value = left - right;
    core::hint::black_box(left);
    core::hint::black_box(right);
    value
}

unsafe fn dual_emit(this_ptr: u32) -> u32 {
    let base = add_f32_ordered(
        unsafe { global_f32(D_G_BASE_A) },
        unsafe { global_f32(D_G_BASE_B) },
    );

    let mut matrix = [0u32; 2];
    let matrix_ptr = matrix.as_mut_ptr() as u32;
    unsafe { callee_cdecl!(D_C_FETCH_MATRIX, u32, matrix_ptr, D_MATRIX_INDEX) };
    let matrix_x = f32::from_bits(matrix[0]);
    let matrix_y = f32::from_bits(matrix[1]);
    unsafe {
        callee_cdecl!(
            D_C_SUBMIT_BASE,
            u32,
            matrix_x.to_bits(),
            matrix_y.to_bits()
        )
    };

    let adjusted_a = sub_f32_ordered(base, unsafe { global_f32(D_G_ADJUST_A) });
    let adjusted_b = sub_f32_ordered(base, unsafe { global_f32(D_G_ADJUST_B) });
    let adjusted_pair = add_f32_ordered(
        adjusted_a,
        unsafe { global_f32(D_G_ADJUST_B) },
    );
    unsafe {
        callee_cdecl!(
            D_C_SET_PAIR,
            u32,
            adjusted_pair.to_bits(),
            adjusted_b.to_bits()
        )
    };

    let mut selector = base.to_bits();
    let selector_ptr = &mut selector as *mut u32 as u32;
    let selected_ptr = unsafe {
        callee_cdecl!(D_C_LOOKUP_SELECTOR, u32, selector_ptr, D_LOOKUP_KEY)
    };
    let selected_value = unsafe { read_u32(selected_ptr) };
    unsafe { callee_cdecl!(D_C_SET_SELECTED, u32, selected_value) };

    let first_mode = unsafe { global::<u8>(D_G_MODE_A).read() };
    let second_mode = unsafe { global::<u8>(D_G_MODE_B).read() };
    let quality = if first_mode != D_SPECIAL_MODE_BYTE && second_mode == 0 {
        D_QUALITY_SPECIAL
    } else {
        D_QUALITY_NORMAL
    };
    unsafe { callee_cdecl!(D_C_SET_QUALITY, u32, quality) };
    unsafe { callee_cdecl!(D_C_SET_PASS, u32, D_PASS_FIRST) };
    unsafe { callee_cdecl!(D_C_SET_SCALE, u32, D_SCALE_BITS) };

    let mut elements = [0u32; 16];
    let elements_ptr = elements.as_mut_ptr() as u32;
    unsafe {
        callee_cdecl!(
            D_C_FETCH_ELEMENTS,
            u32,
            elements_ptr,
            D_PALETTE_ENTRY,
            D_PALETTE_ALPHA_BYTE
        )
    };
    unsafe { callee_cdecl!(D_C_DRAW_MODE, u32, D_DRAW_SOURCE, D_DRAW_FLAGS) };
    unsafe {
        callee_cdecl!(
            D_C_APPLY_EMISSION,
            u32,
            this_ptr.wrapping_add(D_RENDER_COMPONENT_OFFSET),
            elements_ptr
        )
    };
    unsafe { callee_cdecl!(D_C_RESET_STATE, u32,) };

    let mut index = 0u32;
    while index < D_RENDER_RECORD_COUNT {
        unsafe { callee_cdecl!(D_C_SET_PASS, u32, D_PASS_FIRST) };
        let table_a = unsafe {
            global::<u32>(
                D_G_TEXTURE_TABLE_A.wrapping_add(index.wrapping_mul(D_TEXTURE_ENTRY_SIZE))
            )
            .read()
        };
        let token_a = unsafe {
            callee_thiscall!(
                D_C_GET_TEXTURE,
                u32,
                relocated(D_G_TEXTURE_ECX),
                table_a
            )
        };

        let record = this_ptr
            .wrapping_add(D_RENDER_RECORDS_OFFSET)
            .wrapping_add(index.wrapping_mul(D_RENDER_RECORD_SIZE));
        let x = unsafe { read_f32(record) };
        let y = unsafe { read_f32(record.wrapping_add(4)) };
        let first_x = x;
        unsafe {
            callee_cdecl!(
                D_C_EMIT_ENTRY,
                u32,
                first_x.to_bits(),
                y.to_bits(),
                token_a
            )
        };

        unsafe { callee_cdecl!(D_C_SET_PASS, u32, D_PASS_SECOND) };
        let table_b = unsafe {
            global::<u32>(
                D_G_TEXTURE_TABLE_B.wrapping_add(index.wrapping_mul(D_TEXTURE_ENTRY_SIZE))
            )
            .read()
        };
        let token_b = unsafe {
            callee_thiscall!(
                D_C_GET_TEXTURE,
                u32,
                relocated(D_G_TEXTURE_ECX),
                table_b
            )
        };
        unsafe {
            callee_cdecl!(
                D_C_EMIT_ENTRY,
                u32,
                x.to_bits(),
                y.to_bits(),
                token_b
            )
        };
        index = index.wrapping_add(1);
    }

    unsafe { callee_cdecl!(D_C_FINISH, u32,) }
}

export!(thiscall, rw_d743c0(this_ptr: u32) -> u32 {
    unsafe { dual_emit(this_ptr) }
});
