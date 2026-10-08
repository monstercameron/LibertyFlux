//! Proof scope: complete target body within its declared stock fixture.
//! Fixed non-null topology and three scripted collaborators; native effects are untested.
//! Helper-induced type-ID mutation and broader engine topology are not exercised.
//! EFLAGS and MXCSR are unobserved. Invocation-time runner hash binding is absent.
//! Current runner identity is a present-day observation only.
//! All nine declared comparisons passed in 1000 completed original trials.
//! The same-contract semantic mutant failed three times across 5 completed originals.
//! Acceptance binds the preserved tested source and DLL, not a new projection binary.
//! This unbuilt projection removes only the mutant export and comments; helper and positive
//! export code are unchanged. No portable or universal native-equivalence credit is claimed.
use lf_checker_rt::{callee_addr, export, global};

const TABLE_POINTER_VA: u32 = 0x012B_9C78;
const TYPE_INFO_OFFSET: usize = 0x38;
const TYPE_ID_OFFSET: usize = 0x08;
const VELOCITY_TABLE_OFFSET: u32 = 0x70;
const APPLY_VELOCITY_SLOT: usize = 0xA8;

#[inline(never)]
unsafe fn run(this: *mut u8, velocity: *const u32, omit_pre_step: bool) {
    unsafe {
        let record = *((this.add(TYPE_INFO_OFFSET)) as *const u32);
        let type_id = *((record.wrapping_add(TYPE_ID_OFFSET as u32)) as *const u16) as u32;
        if type_id == 0xFFFF {
            return;
        }
        let root = global::<u32>(TABLE_POINTER_VA).read();
        let table = *((root.wrapping_add(VELOCITY_TABLE_OFFSET)) as *const u32);
        let type_word = *((table.wrapping_add(type_id.wrapping_mul(8)).wrapping_add(4)) as *const u32);
        if type_word & 3 == 1 {
            let nonzero = (0..3).any(|i| {
                (core::hint::black_box(*velocity.add(i)) & 0x7FFF_FFFF) != 0
            });
            if nonzero && !omit_pre_step {
                let pre_step: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(callee_addr(1) as usize);
                let _ = pre_step(this as u32);
            }
        }

        let find_object: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(2) as usize);
        let first = find_object(this as u32);
        if first == 0 {
            return;
        }

        let record_after_helper = *((this.add(TYPE_INFO_OFFSET)) as *const u32);
        let type_after_helper =
            *((record_after_helper.wrapping_add(TYPE_ID_OFFSET as u32)) as *const u16);
        if type_after_helper == 0xFFFF {
            return;
        }

        let object = find_object(this as u32);
        let vtable = *(object as *const u32);
        let method = *((vtable.wrapping_add(APPLY_VELOCITY_SLOT as u32)) as *const u32);
        let apply: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(method as usize);
        let _ = apply(object, velocity as u32);
    }
}

export!(thiscall, rw_b25f10(this: *mut u8, velocity: *const u32) -> () {
    unsafe { run(this, velocity, false) }
});

