//! Self-healing Markdown for streaming text.
//!
//! `remend` completes unclosed Markdown delimiters so partially streamed
//! responses can be rendered safely while the response is still arriving.

#[path = "code-block-utils.rs"]
mod code_block_utils;
#[path = "comparison-operator-handler.rs"]
mod comparison_operator_handler;
#[path = "emphasis-handlers.rs"]
mod emphasis_handlers;
#[path = "html-tag-handler.rs"]
mod html_tag_handler;
#[path = "index.rs"]
mod index;
#[path = "inline-code-handler.rs"]
mod inline_code_handler;
#[path = "katex-handler.rs"]
mod katex_handler;
#[path = "link-image-handler.rs"]
mod link_image_handler;
#[path = "patterns.rs"]
mod patterns;
#[path = "setext-heading-handler.rs"]
mod setext_heading_handler;
#[path = "single-tilde-handler.rs"]
mod single_tilde_handler;
#[path = "strikethrough-handler.rs"]
mod strikethrough_handler;
mod utils;

pub use index::{
    FnRemendHandler, LinkMode, RemendHandler, RemendOptions, remend, remend_with_options,
};
pub use utils::{
    find_matching_closing_bracket, find_matching_opening_bracket, is_horizontal_rule,
    is_within_code_block, is_within_html_tag, is_within_link_or_image_url, is_within_math_block,
    is_word_char,
};

pub use emphasis_handlers::{
    count_single_asterisks, count_single_underscores, count_triple_asterisks,
    handle_incomplete_bold, handle_incomplete_bold_italic,
    handle_incomplete_double_underscore_italic, handle_incomplete_single_asterisk_italic,
    handle_incomplete_single_underscore_italic,
};
