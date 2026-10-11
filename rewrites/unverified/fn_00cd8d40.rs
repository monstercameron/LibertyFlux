// original: 0x00CD8D40 CTaskComplexFollowLeaderInFormation::vf18

/// Query the child task kind and dispatch one of the recognized follow-task
/// cases. The threshold at 0x2D4 is signed. Kinds 0x2D4, 0xCB and 0xCA
/// request their matching helper codes; kind 0x11D queries an optional child
/// object and uses the owner state at `this + 0x18 + 0xB80` to choose between
/// helper codes 0xCB and 0x38B. Kinds 0x2DE and 0x2E2 select the parent's
/// virtual callback and helper respectively. Unrecognized kinds return zero.
///
/// `this + 8` points to the child and `this + 0x18` points to the owner-state
/// structure. The child task-kind query is at vtable byte offset 0xC. The
/// optional child pointer is at child offset 0x14 and its query uses the same
/// slot. The parent callback is at vtable offset 0x4C. The incoming target is
/// the sole stack argument; helper calls receive a request code and target.
///
/// Calling convention: thiscall with one 32-bit stack pointer argument. The
/// task-kind callbacks are thiscall with zero stack arguments; the parent
/// callback takes one stack word and the request helper takes two.
lf_checker_rt::export!(thiscall, rw_00cd8d40(this: u32, target: u32) -> u32 {
    const CHILD_POINTER: u32 = 0x08;
    const OWNER_POINTER: u32 = 0x18;
    const OWNER_STATE: u32 = 0xB80;
    const CHILD_OVERRIDE: u32 = 0x14;
    const CHILD_QUERY_SLOT: u32 = 0x0C;
    const PARENT_CALLBACK_SLOT: u32 = 0x4C;
    const TARGET_KIND_THRESHOLD: i32 = 0x2D4;
    const KIND_THRESHOLD: u32 = 0x2D4;
    const KIND_NEAR: u32 = 0xCA;
    const KIND_FAR: u32 = 0xCB;
    const KIND_NESTED: u32 = 0x11D;
    const KIND_CALLBACK: u32 = 0x2DE;
    const KIND_HELPER: u32 = 0x2E2;
    const NESTED_KIND_NULL: u32 = 0xC8;
    const NESTED_KIND_NEAR: u32 = 0x38B;
    const NESTED_KIND_FAR: u32 = 0x3B6;
    const OWNER_FIRST_STATE: u32 = 0;
    const OWNER_SECOND_STATE: u32 = 1;
    const REQUEST_NEAR: u32 = 0xCA;
    const REQUEST_FAR: u32 = 0xCB;
    const REQUEST_DEFAULT: u32 = 0x38B;
    const REQUEST_TASK: u32 = 4;

    unsafe {
        let owner = ((this + OWNER_POINTER) as *const u32).read_unaligned();
        let owner_state = ((owner + OWNER_STATE) as *const u32).read_unaligned();
        let child = ((this + CHILD_POINTER) as *const u32).read_unaligned();
        let child_vtable = (child as *const u32).read_unaligned();
        let child_query_address =
            ((child_vtable + CHILD_QUERY_SLOT) as *const u32).read_unaligned();
        let child_query: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(child_query_address as usize);
        let kind = child_query(child);

        if (kind as i32) > TARGET_KIND_THRESHOLD {
            if kind == KIND_CALLBACK {
                let parent_vtable = (this as *const u32).read_unaligned();
                let callback_address =
                    ((parent_vtable + PARENT_CALLBACK_SLOT) as *const u32).read_unaligned();
                let callback: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(callback_address as usize);
                return callback(this, target);
            }
            if kind == KIND_HELPER {
                return lf_checker_rt::callee_thiscall!(
                    REQUEST_TASK, u32, this, REQUEST_DEFAULT, target
                );
            }
            return 0;
        }

        if kind == KIND_THRESHOLD {
            return lf_checker_rt::callee_thiscall!(REQUEST_TASK, u32, this, REQUEST_DEFAULT, target);
        }
        if kind == KIND_NEAR {
            return lf_checker_rt::callee_thiscall!(REQUEST_TASK, u32, this, REQUEST_NEAR, target);
        }
        if kind == KIND_FAR {
            return lf_checker_rt::callee_thiscall!(REQUEST_TASK, u32, this, REQUEST_DEFAULT, target);
        }
        if kind != KIND_NESTED {
            return 0;
        }

        let nested_object = ((child + CHILD_OVERRIDE) as *const u32).read_unaligned();
        let nested_kind = if nested_object == 0 {
            NESTED_KIND_NULL
        } else {
            let nested_vtable = (nested_object as *const u32).read_unaligned();
            let query_address = ((nested_vtable + CHILD_QUERY_SLOT) as *const u32).read_unaligned();
            let query: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(query_address as usize);
            query(nested_object)
        };

        if nested_kind == NESTED_KIND_NEAR {
            let request = if owner_state == OWNER_FIRST_STATE || owner_state == OWNER_SECOND_STATE {
                REQUEST_FAR
            } else {
                REQUEST_DEFAULT
            };
            return lf_checker_rt::callee_thiscall!(REQUEST_TASK, u32, this, request, target);
        }
        if nested_kind == NESTED_KIND_FAR {
            return lf_checker_rt::callee_thiscall!(REQUEST_TASK, u32, this, REQUEST_DEFAULT, target);
        }
        0
    }
});
