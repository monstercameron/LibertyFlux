//! The verified rewrites of the instances, each behind one function of the same shape.
//!
//! Entry `i` calls the rewrite of entry `i` of `lf_core::constant_writes::instances::INSTANCES`.

/// The rewrite of `reset_state_b`, called with the object at `base`; the result is widened to a word.
pub fn call_0(base: *mut u8) -> u32 {
    crate::rewrites::m_0094eeb0::rw_0094eeb0(base)
}

/// The rewrite of `ui_element_init_core`, called with the object at `base`; the result is widened to a word.
pub fn call_1(base: *mut u8) -> u32 {
    crate::rewrites::m_008bf490::rw_008BF490(base as usize as u32)
}

/// The rewrite of `Input_ResetDevice`, called with the object at `base`; the result is widened to a word.
pub fn call_2(base: *mut u8) -> u32 {
    crate::rewrites::m_008f6790::rw_008f6790(base as usize as u32)
}

/// The rewrite of `HandleRec_Init`, called with the object at `base`; the result is widened to a word.
pub fn call_3(base: *mut u8) -> u32 {
    crate::rewrites::m_00e3eda0::rw_00e3eda0(base)
}

/// The rewrite of `tag6_clear`, called with the object at `base`; the result is widened to a word.
pub fn call_4(base: *mut u8) -> u32 {
    crate::rewrites::m_00b2f880::rw_00b2f880(base)
}

/// The rewrite of `ccam_sector_init`, called with the object at `base`; the result is widened to a word.
pub fn call_5(base: *mut u8) -> u32 {
    crate::rewrites::m_00d5ac00::rw_00d5ac00(base as usize as u32)
}

/// The rewrite of `clear_field_0ee0`, called with the object at `base`; the result is widened to a word.
pub fn call_6(base: *mut u8) -> u32 {
    crate::rewrites::m_00d78990::rw_00d78990(base as usize as u32)
}

/// The rewrite of `targeting_float_reset`, called with the object at `base`; the result is widened to a word.
pub fn call_7(base: *mut u8) -> u32 {
    crate::rewrites::m_00d29770::rw_00d29770(base as usize as u32)
}

/// The rewrite of `rage::crCreatureComponent::vf2`, called with the object at `base`; the result is widened to a word.
pub fn call_8(base: *mut u8) -> u32 {
    crate::rewrites::m_006935d0::rw_006935D0(base as usize as u32)
}

/// The rewrite of `rage::crFrameFilterN::vf6`, called with the object at `base`; the result is widened to a word.
pub fn call_9(base: *mut u8) -> u32 {
    crate::rewrites::m_00694e00::rw_00694E00(base as usize as u32)
}

/// The call table, in the order of the instances table.
pub const CALLS: [fn(*mut u8) -> u32; 10] = [
    call_0, call_1, call_2, call_3, call_4, call_5, call_6, call_7, call_8, call_9,
];
