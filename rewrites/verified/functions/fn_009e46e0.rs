//! Proof scope: complete guarded predicate body within the declared object/table fixture.
//! Both collaborators are recorder stubs; native helper effects are untested.
//! Signed indices cover only zero and one, with null or declared helper-object table entries.
//! Guard, enabled and shortcut bytes and helper AL answers use the declared bounded values.
//! All configured comparisons passed; arbitrary engine states and general stack/register coverage are unknown.
//! The historical index-pin error remains excluded. Only the mutant export is removed;
//! positive helper bytes are unchanged, with the retained private switch selected false.
//! The source projection is unbuilt; acceptance binds the tested source and preserved DLL.
#![allow(unsafe_code)]

use core::ptr;
use lf_checker_rt::{callee_thiscall, export};

const GLOBAL_FLAG_FILE_VA: u32 = 0x012B_41C6;
const GLOBAL_TABLE_FILE_VA: u32 = 0x0129_5CD8;
const OBJECT_ENABLED_OFFSET: usize = 0x266;
const OBJECT_SHORTCUT_OFFSET: usize = 0xA60;
const OBJECT_INDEX_OFFSET: usize = 0x2E;
const VTABLE_SLOT_74_OFFSET: usize = 0x128;
const HELPER_OBJECT_DELTA: u32 = 0x120;
const DIRECT_HELPER_ARGUMENT: u32 = 9;

/// Read the global guard and enabled bit, then evaluate the virtual predicate,
/// the byte-2 shortcut, and the indexed helper. The two calls are intercepted
/// by contract callees: slot 74 is thiscall/0/AL, and the E8 helper is
/// thiscall/1/AL with stack argument 9 and ECX equal to table_entry + 0x120.
fn body(this: u32, invert_shortcut: bool) -> u8 {
    let global_flag = unsafe {
        ptr::read_volatile(lf_checker_rt::global::<u8>(GLOBAL_FLAG_FILE_VA))
    };
    if global_flag != 0 {
        return 0;
    }

    let object = this as usize as *mut u8;
    let enabled = unsafe { ptr::read_volatile(object.add(OBJECT_ENABLED_OFFSET)) };
    if enabled & 1 == 0 {
        return 0;
    }

    let virtual_answer: u8 = callee_thiscall!(1, u8, this);
    if virtual_answer != 0 {
        return 1;
    }

    let shortcut_value = unsafe {
        ptr::read_volatile(object.add(OBJECT_SHORTCUT_OFFSET))
    };
    let shortcut_matches = if invert_shortcut {
        shortcut_value != 2
    } else {
        shortcut_value == 2
    };
    if shortcut_matches {
        return 1;
    }

    let table_index = unsafe {
        i32::from(ptr::read_unaligned(
            object.add(OBJECT_INDEX_OFFSET).cast::<i16>()
        ))
    };
    let table_base = lf_checker_rt::global::<u8>(GLOBAL_TABLE_FILE_VA);
    let table_offset = table_index.wrapping_mul(4) as isize;
    let table_entry = table_base.wrapping_offset(table_offset).cast::<u32>();
    let table_object = unsafe { ptr::read_unaligned(table_entry) };
    if table_object == 0 {
        return 0;
    }

    let helper_this = table_object.wrapping_add(HELPER_OBJECT_DELTA);
    let direct_answer: u8 = callee_thiscall!(
        2,
        u8,
        helper_this,
        DIRECT_HELPER_ARGUMENT
    );
    u8::from(direct_answer != 0)
}

export!(thiscall, rw_fn_009e46e0(this: u32) -> u8 {
    body(this, false)
});
