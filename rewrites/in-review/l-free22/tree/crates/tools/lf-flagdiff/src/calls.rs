//! The verified rewrites of the instances, each behind one function of the same shape.
//!
//! Entry `i` calls the rewrite of entry `i` of `lf_core::word_test::instances::PREDICATES`. The result
//! is the rewrite's return value, widened to a word.

/// The rewrite of `audPoliceScanner::is_scanning`, called with the object at `base` and the parameter.
pub fn call_0(base: *mut u8, parameter: u32) -> u32 {
    u32::from(crate::rewrites::m_0098c170::rw_aud_police_scanner_is_scanning(base as usize as u32))
}

/// The rewrite of `check_state_equals_one`, called with the object at `base` and the parameter.
pub fn call_1(base: *mut u8, parameter: u32) -> u32 {
    u32::from(crate::rewrites::m_009fbac0::rw_rs227_009fbac0(base as usize as u32))
}

/// The rewrite of `audStaticRadioEmitter::vf5`, called with the object at `base` and the parameter.
pub fn call_2(base: *mut u8, parameter: u32) -> u32 {
    u32::from(crate::rewrites::m_00985d60::rw_00985d60(base as usize as u32, parameter))
}

/// The rewrite of `object_field_10_not_200`, called with the object at `base` and the parameter.
pub fn call_3(base: *mut u8, parameter: u32) -> u32 {
    u32::from(crate::rewrites::m_00b4bad0::rw_00B4BAD0(base as usize as u32))
}

/// The rewrite of `report_whether_the_streaming_state_word`, a static-word test: it reads the words, so its arguments are unused.
pub fn call_4(_base: *mut u8, _parameter: u32) -> u32 {
    u32::from(crate::rewrites::m_00942840::rw_00942840())
}

/// The rewrite of `NativeImpl_GET_PLAYER_HAS_TRACKS`, a static-word test: it reads the words, so its arguments are unused.
pub fn call_5(_base: *mut u8, _parameter: u32) -> u32 {
    u32::from(crate::rewrites::m_009ccd80::rw_009ccd80())
}

/// The call table, in the order of the instances table.
pub const CALLS: [fn(*mut u8, u32) -> u32; 6] = [call_0, call_1, call_2, call_3, call_4, call_5];
