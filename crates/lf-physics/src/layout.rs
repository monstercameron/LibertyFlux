//! Fixed 32-bit memory layouts for physics.
//!
//! Convention (see `layout-convention.md` in the project notes):
//! - Every type is `#[repr(C)]` and uses `lf_core::Ptr32` instead of raw
//!   pointers or references, so the layout is identical for 32-bit and
//!   64-bit builds.
//! - Every type carries an `lf_core::assert_size!` check and every field a
//!   caller depends on carries an `lf_core::assert_offset!` check.
//! - The 32-bit build compares these layouts against the original; the
//!   64-bit build converts between them and portable types at this
//!   module's boundary. Conversion helpers live here, next to the types.
//!
//! Nothing here yet: layouts are added as structures are recovered.
