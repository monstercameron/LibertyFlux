#![allow(clippy::all)]

// original: 0x00C66BE0 CCutsceneObject::vf33
//
// Applies the object's pending cutscene state, then rebuilds the pointed-to
// 4-by-4 transform from two scripted double-valued adjustments. It returns
// the result of the final state hook, invoking the fallback hook when that
// result is zero. The ten queued identifiers skip the sentinel value, and
// the signed table index selects the row whose state controls a three-call
// update block. The matrix calculations keep each scalar operation in the
// original order so every intermediate is rounded to one f32 value.

const OBJECT_VTABLE: u32 = 0;
const OBJECT_PARTS: u32 = 0x290;
const OBJECT_CHILD: u32 = 0x34;
const OBJECT_FALLBACK: u32 = 0x100;
const OBJECT_FLAGS: u32 = 0x2ab;
const OBJECT_TABLE_INDEX: u32 = 0x2e;
const OBJECT_QUEUE: u32 = 0x2b8;
const QUEUE_COUNT: u32 = 10;
const TABLE_FILE_VA: u32 = 0x1295_cd8;
const ROW_STATE: u32 = 0xcc;
const STATE_VALUE: u32 = 0x18c;

const SLOT_READY: u32 = 0x24;
const SLOT_ACTIVATE: u32 = 0x28;
const SLOT_MATRIX_SOURCE: u32 = 0xa0;
const SLOT_MATRIX_RESULT: u32 = 0xe0;
const SLOT_FALLBACK: u32 = 0xcc;
const SLOT_PART_READY: u32 = 4;
const SLOT_CHILD_ACTIVATE: u32 = 0xc;

const HOOK_READY: u32 = 1;
const HOOK_PART_READY: u32 = 2;
const HOOK_ACTIVATE: u32 = 3;
const HOOK_CHILD_ACTIVATE: u32 = 4;
const HOOK_QUEUE: u32 = 5;
const HOOK_ROW_UPDATE: u32 = 6;
const HOOK_MATRIX_SOURCE: u32 = 7;
const HOOK_MATRIX_RESULT: u32 = 8;
const HOOK_STATE_ID: u32 = 9;
const HOOK_MATRIX: u32 = 10;
const HOOK_ANGLE_A: u32 = 11;
const HOOK_ANGLE_B: u32 = 12;
const HOOK_FINAL_STATE: u32 = 13;
const HOOK_FINAL_FALLBACK: u32 = 14;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
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
unsafe fn write_u32(address: u32, value: u32) {
    unsafe { (address as *mut u32).write_unaligned(value) }
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
fn scalar_mul(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) * core::hint::black_box(right)
}

#[inline(never)]
fn scalar_add(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) + core::hint::black_box(right)
}

#[inline(never)]
fn scalar_sub(left: f32, right: f32) -> f32 {
    core::hint::black_box(left) - core::hint::black_box(right)
}

#[inline(never)]
fn double_answer_to_f32(bits: u64) -> f32 {
    let value = core::hint::black_box(f64::from_bits(bits));
    core::hint::black_box(value) as f32
}

unsafe fn virtual_thiscall0(vtable: u32, slot: u32, this: u32) -> u32 {
    let target = unsafe { read_u32(vtable.wrapping_add(slot)) };
    let function: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(target as usize) };
    function(this)
}

unsafe fn virtual_thiscall1(vtable: u32, slot: u32, this: u32, arg0: u32) -> u32 {
    let target = unsafe { read_u32(vtable.wrapping_add(slot)) };
    let function: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(target as usize) };
    function(this, arg0)
}

fn rewrite(this: u32) -> u32 {
    unsafe {
        let object_vtable = read_u32(this.wrapping_add(OBJECT_VTABLE));
        if virtual_thiscall0(object_vtable, SLOT_READY, this) as u8 != 0 {
            let parts = read_u32(this.wrapping_add(OBJECT_PARTS));
            let parts_vtable = read_u32(parts);
            let _ = virtual_thiscall0(parts_vtable, SLOT_PART_READY, parts);
        }

        if virtual_thiscall0(object_vtable, SLOT_ACTIVATE, this) as u8 != 0 {
            let child_owner = read_u32(this.wrapping_add(OBJECT_CHILD));
            let child = read_u32(child_owner.wrapping_add(4));
            let child_vtable = read_u32(child);
            let _ = virtual_thiscall1(child_vtable, SLOT_CHILD_ACTIVATE, child, this);

            let mut queue_index = 0;
            while queue_index < QUEUE_COUNT {
                let value = read_u32(
                    this.wrapping_add(OBJECT_QUEUE)
                        .wrapping_add(queue_index.wrapping_mul(4)),
                );
                if value != u32::MAX {
                    let _ = lf_checker_rt::callee_thiscall!(HOOK_QUEUE, u32, this, value);
                }
                queue_index += 1;
            }

            let table_index = read_u16(this.wrapping_add(OBJECT_TABLE_INDEX)) as i16 as i32;
            let table_address = lf_checker_rt::relocated(TABLE_FILE_VA);
            let row = read_u32(table_address.wrapping_add((table_index * 4) as u32));
            let row_state = read_u32(row.wrapping_add(ROW_STATE));
            if read_u32(row_state.wrapping_add(STATE_VALUE)) != 0 {
                let _ = lf_checker_rt::callee_thiscall!(HOOK_ROW_UPDATE, u32, this, 10, row);
                let _ = lf_checker_rt::callee_thiscall!(HOOK_ROW_UPDATE, u32, this, 12, row);
                let _ = lf_checker_rt::callee_thiscall!(HOOK_ROW_UPDATE, u32, this, 14, row);
            }
        }

        let mut matrix_source = virtual_thiscall0(object_vtable, SLOT_MATRIX_SOURCE, this);
        if matrix_source != 0 {
            matrix_source = virtual_thiscall0(object_vtable, SLOT_MATRIX_SOURCE, this);
            let source_vtable = read_u32(matrix_source);
            matrix_source = virtual_thiscall0(source_vtable, SLOT_MATRIX_RESULT, matrix_source);
        } else {
            matrix_source = read_u32(this.wrapping_add(OBJECT_FALLBACK));
        }

        let state_word = read_u32(matrix_source.wrapping_add(4));
        let state_id = lf_checker_rt::callee_cdecl!(HOOK_STATE_ID, u32, state_word, 0);
        let state_id = if state_id == u32::MAX { 0 } else { state_id };
        let matrix = lf_checker_rt::callee_thiscall!(HOOK_MATRIX, u32, this, state_id);

        if read_u8(this.wrapping_add(OBJECT_FLAGS)) == 0 {
            write_f32(matrix.wrapping_add(0x30), 0.0);
            write_f32(matrix.wrapping_add(0x34), 0.0);
            write_f32(matrix.wrapping_add(0x38), 0.0);
            // The original reads a stack local that it did not initialize.
            // The contract supplies zero for that word on both sides.
            write_f32(matrix.wrapping_add(0x3c), 0.0);
        }

        write_f32(matrix, 1.0);
        write_f32(matrix.wrapping_add(4), 0.0);
        write_f32(matrix.wrapping_add(8), 0.0);
        write_f32(matrix.wrapping_add(0x10), 0.0);
        write_f32(matrix.wrapping_add(0x14), 1.0);
        write_f32(matrix.wrapping_add(0x18), 0.0);
        write_f32(matrix.wrapping_add(0x20), 0.0);
        write_f32(matrix.wrapping_add(0x24), 0.0);
        write_f32(matrix.wrapping_add(0x28), 1.0);

        let angle_a_bits = lf_checker_rt::callee_cdecl!(HOOK_ANGLE_A, u64, 0, 0);
        let angle_a = double_answer_to_f32(angle_a_bits);
        let angle_b_bits = lf_checker_rt::callee_cdecl!(HOOK_ANGLE_B, u64, 0, 0);
        let angle_b = double_answer_to_f32(angle_b_bits);

        let mut x1 = read_f32(matrix.wrapping_add(0x14));
        let mut x5 = read_f32(matrix.wrapping_add(0x18));
        let mut x6 = angle_b;
        let mut x0 = scalar_mul(read_f32(matrix.wrapping_add(0x10)), angle_a);
        let mut x2 = 0.0f32;
        let mut x4 = x6;
        x4 = scalar_mul(x4, x2);
        let mut x3 = x4;
        x3 = scalar_add(x3, x0);
        x0 = 0.0;
        x2 = scalar_mul(angle_a, x0);
        write_f32(matrix.wrapping_add(0x28), angle_a);
        x1 = scalar_mul(x1, angle_a);
        x4 = scalar_add(x4, x1);
        x5 = scalar_mul(x5, angle_a);
        write_f32(matrix.wrapping_add(0x20), x2);
        write_f32(matrix.wrapping_add(0x24), x2);
        x1 = scalar_mul(read_f32(matrix.wrapping_add(0x10)), x6);
        x0 = x2;
        x5 = scalar_add(x5, x6);
        x0 = scalar_sub(x0, x1);
        let angle_b_saved = x6;
        write_f32(matrix.wrapping_add(0x20), x0);
        x0 = scalar_mul(read_f32(matrix.wrapping_add(0x14)), x6);
        x2 = scalar_sub(x2, x0);
        x0 = angle_a;
        write_f32(matrix.wrapping_add(0x24), x2);
        x1 = scalar_mul(read_f32(matrix.wrapping_add(0x18)), x6);
        x0 = scalar_sub(x0, x1);
        write_f32(matrix.wrapping_add(0x28), x0);
        write_f32(matrix.wrapping_add(0x10), x3);
        x3 = x6;
        write_f32(matrix.wrapping_add(0x18), x5);
        write_f32(matrix.wrapping_add(0x14), x4);

        x2 = read_f32(matrix);
        x5 = read_f32(matrix.wrapping_add(4));
        x6 = read_f32(matrix.wrapping_add(8));
        x0 = x3;
        x0 = scalar_mul(x0, read_f32(matrix.wrapping_add(0x20)));
        x2 = scalar_mul(x2, angle_a);
        x5 = scalar_mul(x5, angle_a);
        x2 = scalar_sub(x2, x0);
        x0 = read_f32(matrix.wrapping_add(0x24));
        x0 = scalar_mul(x0, x3);
        x6 = scalar_mul(x6, angle_a);
        x5 = scalar_sub(x5, x0);
        x0 = read_f32(matrix.wrapping_add(0x28));
        x0 = scalar_mul(x0, x3);
        x6 = scalar_sub(x6, x0);
        x0 = angle_a;
        x0 = scalar_mul(x0, read_f32(matrix.wrapping_add(0x20)));
        write_f32(matrix.wrapping_add(0x20), x0);
        x0 = read_f32(matrix.wrapping_add(0x24));
        x0 = scalar_mul(x0, angle_a);
        write_f32(matrix.wrapping_add(0x24), x0);
        x0 = read_f32(matrix.wrapping_add(0x28));
        x0 = scalar_mul(x0, angle_a);
        write_f32(matrix.wrapping_add(0x28), x0);
        x0 = read_f32(matrix);
        x0 = scalar_mul(x0, x3);
        let mut x4 = x3;
        x0 = scalar_add(x0, read_f32(matrix.wrapping_add(0x20)));
        write_f32(matrix.wrapping_add(0x20), x0);
        x0 = read_f32(matrix.wrapping_add(4));
        x0 = scalar_mul(x0, x3);
        x0 = scalar_add(x0, read_f32(matrix.wrapping_add(0x24)));
        write_f32(matrix.wrapping_add(0x24), x0);
        x0 = read_f32(matrix.wrapping_add(8));
        x0 = scalar_mul(x0, x3);
        x0 = scalar_add(x0, read_f32(matrix.wrapping_add(0x28)));
        write_f32(matrix.wrapping_add(0x28), x0);
        write_f32(matrix, x2);
        write_f32(matrix.wrapping_add(4), x5);
        write_f32(matrix.wrapping_add(8), x6);
        x4 = scalar_mul(x4, read_f32(matrix.wrapping_add(0x10)));
        x3 = read_f32(matrix.wrapping_add(0x14));
        x3 = scalar_mul(x3, angle_b_saved);
        x2 = scalar_mul(x2, angle_a);
        x0 = angle_a;
        x0 = scalar_mul(x0, read_f32(matrix.wrapping_add(0x10)));
        x4 = scalar_add(x4, x2);
        x2 = read_f32(matrix.wrapping_add(0x18));
        write_f32(matrix.wrapping_add(0x10), x0);
        x0 = read_f32(matrix.wrapping_add(0x14));
        x0 = scalar_mul(x0, angle_a);
        x5 = scalar_mul(x5, angle_a);
        write_f32(matrix.wrapping_add(0x14), x0);
        x0 = read_f32(matrix.wrapping_add(0x18));
        x0 = scalar_mul(x0, angle_a);
        x3 = scalar_add(x3, x5);
        x5 = angle_b_saved;
        write_f32(matrix.wrapping_add(0x18), x0);
        x1 = read_f32(matrix);
        x0 = read_f32(matrix.wrapping_add(0x10));
        x1 = scalar_mul(x1, x5);
        x6 = scalar_mul(x6, angle_a);
        x0 = scalar_sub(x0, x1);
        x2 = scalar_mul(x2, x5);
        write_f32(matrix.wrapping_add(0x10), x0);
        x1 = read_f32(matrix.wrapping_add(4));
        x0 = read_f32(matrix.wrapping_add(0x14));
        x1 = scalar_mul(x1, x5);
        x2 = scalar_add(x2, x6);
        x0 = scalar_sub(x0, x1);
        write_f32(matrix.wrapping_add(0x14), x0);
        x1 = read_f32(matrix.wrapping_add(8));
        x0 = read_f32(matrix.wrapping_add(0x18));
        x1 = scalar_mul(x1, x5);
        x0 = scalar_sub(x0, x1);
        write_f32(matrix.wrapping_add(0x18), x0);
        write_f32(matrix, x4);
        write_f32(matrix.wrapping_add(4), x3);
        write_f32(matrix.wrapping_add(8), x2);

        let final_state = lf_checker_rt::callee_thiscall!(HOOK_FINAL_STATE, u32, this);
        if final_state == 0 {
            let _ = virtual_thiscall0(object_vtable, SLOT_FALLBACK, this);
        }
        final_state
    }
}

lf_checker_rt::export!(thiscall, rw_00C66BE0(this: u32) -> u32 {
    rewrite(this)
});
