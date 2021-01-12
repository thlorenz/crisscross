#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    // TODO(thlorenz): prepare toml to publish
    // clippy::cargo
)]
// clippy::restriction,
#![deny(
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::else_if_without_else,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::indexing_slicing,
    clippy::inline_asm_x86_att_syntax,
    clippy::inline_asm_x86_intel_syntax,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::shadow_same,
    clippy::string_add,
    clippy::todo,
    clippy::unimplemented,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm,
    clippy::wrong_pub_self_convention
)]
#![allow(
    // TODO(thlorenz): add docs
    missing_crate_level_docs,
    missing_doc_code_examples,
    clippy::similar_names,
    clippy::module_name_repetitions,
    clippy::erasing_op,
    clippy::cargo_common_metadata
)]
mod grid;
mod intersections;
mod intersections_iter;
mod position;
mod tile_raycaster;
mod util;

pub use grid::Grid;
pub use position::TilePosition;
pub use tile_raycaster::TileRaycaster;
