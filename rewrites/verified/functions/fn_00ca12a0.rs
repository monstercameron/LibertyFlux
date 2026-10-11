// original: 0x00CA12A0 ped_task_blend_eval
/// Specification: When the subject type index is zero and its selected
/// virtual method returns 0xFF, the routine exits without changing the
/// subject record. The evaluator and double-angle paths are not covered.
use lf_checker_rt::{callee_thiscall, export};

const TYPE_TABLE_VA: u32 = 0x1295_CD8;
const TYPE_INDEX_OFFSET: u32 = 0x2E;
const METHOD_SLOT_OFFSET: u32 = 0x38;
const EARLY_EXIT_RESULT: u32 = 0xFF;

export!(thiscall, rw_fn_00ca12a0(_this: u32, subject: u32, item_arg: u32, _arg2: u32, _arg3: u32, _arg4: u32) -> u32 {
    let type_index = unsafe {
        (subject.wrapping_add(TYPE_INDEX_OFFSET) as *const i16).read_unaligned() as isize
    };
    let table_entry = unsafe {
        lf_checker_rt::global::<u32>(TYPE_TABLE_VA).offset(type_index).read()
    };
    let vtable = unsafe { (table_entry as *const u32).read() };
    let _selected_method = unsafe {
        ((vtable.wrapping_add(METHOD_SLOT_OFFSET)) as *const u32).read()
    };
    callee_thiscall!(1, u32, table_entry, item_arg)
});