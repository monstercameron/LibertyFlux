/// Proof scope: scripted factory and lookup helpers with synthetic state.
/// Null state and helper-null branches remain untested; full-data is disabled.
/// The candidate reads a state word before its read-only lookup helper;
/// the native caller reads it after. Concurrent mutation is outside this proof.
use lf_checker_rt::{callee_cdecl, export, global};

#[repr(C)]
struct NativeCallContext {
    return_slot: *mut u32,
    _reserved: u32,
    arguments: *const u32,
}

const SCRIPT_FACTORY_SLOT_FILE_VA: u32 = 0x0110_B728;
const CURRENT_SCRIPT_STATE_FILE_VA: u32 = 0x01BB_54DC;
const TABLE_WORDS_START: u32 = 0x34;
const TABLE_WORDS_END: u32 = 0x44;

unsafe fn execute(ctx: *const NativeCallContext, wrong_copy: bool) -> u32 {
    unsafe {
        let arguments = (*ctx).arguments;
        let script_data = *arguments;
        let requested_size = *arguments.add(1);
        let stack_size = if requested_size != 0 {
            requested_size
        } else {
            0x200
        };

        let factory_address = global::<u32>(SCRIPT_FACTORY_SLOT_FILE_VA).read();
        let factory: unsafe extern "cdecl" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(factory_address as usize);
        let script = factory(script_data, 0, 0, stack_size);

        if script == 0 {
            let state = global::<u32>(CURRENT_SCRIPT_STATE_FILE_VA).read();
            if state != 0 {
                *((state.wrapping_add(12)) as *mut u32) = 1;
            }
            return state;
        }

        let return_slot = (*ctx).return_slot;
        *return_slot = script;

        let state = global::<u32>(CURRENT_SCRIPT_STATE_FILE_VA).read();
        if state == 0 {
            return return_slot as u32;
        }

        let mut last_table = 0;
        for offset in (TABLE_WORDS_START..TABLE_WORDS_END).step_by(4usize) {
            let table = callee_cdecl!(2, u32, script);
            let destination = (table.wrapping_add(offset)) as *mut u32;
            if *destination == 0 {
                let source = *((state.wrapping_add(offset)) as *const u32);
                last_table = callee_cdecl!(2, u32, script);
                let value = if wrong_copy { 0 } else { source };
                *((last_table.wrapping_add(offset)) as *mut u32) = value;
            } else {
                last_table = table;
            }
        }
        last_table
    }
}

export!(cdecl, rw_0086ea80(ctx: *const NativeCallContext) -> u32 {
    unsafe { execute(ctx, false) }
});
