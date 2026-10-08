/// Proof scope: eight active comparison families; return equality is disabled.
/// The known direct caller ignores the return register; the prototype is unknown.
/// Helpers and the virtual method are scripted fixtures, not proven implementations.
/// The manager/table fixture covers four type identifiers and the tested null states.
/// Virtual argument pointer identity is skipped; its three float words are snapshotted.
/// The source projection preserves the tested floating-point operation order.
use lf_checker_rt as lf_k2_rt;
use lf_k2_rt::{export, global};

const TYPE_TABLE_ROOT_VA: u32 = 0x012B_9C78;
const TYPE_TABLE_PTR_OFFSET: usize = 0x70;
const TYPE_ROW_STRIDE: usize = 8;
const TYPE_ROW_VALUE_OFFSET: usize = 4;
const VIRTUAL_SLOT_OFFSET: usize = 0x88;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn read_f32(address: u32) -> f32 {
    unsafe { (address as *const f32).read_unaligned() }
}

#[inline(always)]
fn cross_second_by_first(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    // Keep the original multiply/subtract sequence separate and ordered.
    let x_left = core::hint::black_box(a[2] * b[1]);
    let x_right = core::hint::black_box(a[1] * b[2]);
    let x = core::hint::black_box(x_left - x_right);

    let y_left = core::hint::black_box(a[0] * b[2]);
    let y_right = core::hint::black_box(a[2] * b[0]);
    let y = core::hint::black_box(y_left - y_right);

    let z_left = core::hint::black_box(a[1] * b[0]);
    let z_right = core::hint::black_box(a[0] * b[1]);
    let z = core::hint::black_box(z_left - z_right);
    [x, y, z]
}

unsafe fn apply_cross_force(this_ptr: u32, a_ptr: u32, b_ptr: u32) {
    let type_data = unsafe { read_u32(this_ptr.wrapping_add(0x38)) };
    if type_data == 0 {
        return;
    }
    let type_id = unsafe { (type_data.wrapping_add(8) as *const u16).read_unaligned() } as usize;
    let manager = unsafe { global::<u32>(TYPE_TABLE_ROOT_VA).read() };
    let table = unsafe { read_u32(manager.wrapping_add(TYPE_TABLE_PTR_OFFSET as u32)) };
    let entry_at = table
        .wrapping_add((type_id * TYPE_ROW_STRIDE + TYPE_ROW_VALUE_OFFSET) as u32);
    let entry_flags = unsafe { read_u32(entry_at) };
    if entry_flags & 3 == 1 {
        let _ = lf_k2_rt::callee_thiscall!(1, u32, this_ptr);
    }

    let first_object = lf_k2_rt::callee_thiscall!(2, u32, this_ptr);
    if first_object == 0 {
        return;
    }

    let a = unsafe { [read_f32(a_ptr), read_f32(a_ptr.wrapping_add(4)), read_f32(a_ptr.wrapping_add(8))] };
    let b = unsafe { [read_f32(b_ptr), read_f32(b_ptr.wrapping_add(4)), read_f32(b_ptr.wrapping_add(8))] };
    let cross = cross_second_by_first(a, b);

    let object = lf_k2_rt::callee_thiscall!(2, u32, this_ptr);
    let vtable = unsafe { read_u32(object) };
    let method_address = unsafe { read_u32(vtable.wrapping_add(VIRTUAL_SLOT_OFFSET as u32)) };
    let method: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(method_address as usize) };
    let _ = method(object, cross.as_ptr() as u32);
}

lf_k2_rt::export!(thiscall, rw_a_q140_b23760(this_ptr: u32, a_ptr: u32, b_ptr: u32) -> () {
    unsafe { apply_cross_force(this_ptr, a_ptr, b_ptr) }
});
