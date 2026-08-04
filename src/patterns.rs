//! Pattern definitions kept in sync with upstream `patterns.ts`.
//!
//! The Rust implementation uses byte-safe scanners instead of a regular
//! expression runtime, but keeping the upstream expressions together makes
//! source synchronization and review straightforward.

#![allow(dead_code)]

pub(crate) const BOLD_PATTERN: &str = r"(\*\*)([^*]*\*?)$";
pub(crate) const ITALIC_PATTERN: &str = r"(__)([^_]*?)$";
pub(crate) const BOLD_ITALIC_PATTERN: &str = r"(\*\*\*)([^*]*?)$";
pub(crate) const SINGLE_ASTERISK_PATTERN: &str = r"(\*)([^*]*?)$";
pub(crate) const SINGLE_UNDERSCORE_PATTERN: &str = r"(_)([^_]*?)$";
pub(crate) const INLINE_CODE_PATTERN: &str = r"(`)([^`]*?)$";
pub(crate) const STRIKETHROUGH_PATTERN: &str = r"(~~)([^~]*?)$";
pub(crate) const WHITESPACE_OR_MARKERS_PATTERN: &str = r"^[\s_~*`]*$";
pub(crate) const LIST_ITEM_PATTERN: &str = r"^[\s]*[-*+][\s]+$";
pub(crate) const LETTER_NUMBER_UNDERSCORE_PATTERN: &str = r"[\p{L}\p{N}_]";
pub(crate) const INLINE_TRIPLE_BACKTICK_PATTERN: &str = r"^```[^`\n]*```?$";
pub(crate) const FOUR_OR_MORE_ASTERISKS_PATTERN: &str = r"^\*{4,}$";
pub(crate) const LINK_IMAGE_PATTERN: &str = r"(!?\[)([^\]]*?)$";
pub(crate) const INCOMPLETE_LINK_URL_PATTERN: &str = r"(!?)\[([^\]]+)\](\([^)]+)$";
pub(crate) const HALF_COMPLETE_UNDERSCORE_PATTERN: &str = r"(__)([^_]+)_$";
pub(crate) const HALF_COMPLETE_TILDE_PATTERN: &str = r"(~~)([^~]+)~$";
pub(crate) const DOUBLE_UNDERSCORE_GLOBAL_PATTERN: &str = r"__";
pub(crate) const DOUBLE_TILDE_GLOBAL_PATTERN: &str = r"~~";
