use std::fmt;

use crate::comparison_operator_handler::handle_comparison_operators;
use crate::emphasis_handlers::{
    handle_incomplete_bold, handle_incomplete_bold_italic,
    handle_incomplete_double_underscore_italic, handle_incomplete_single_asterisk_italic,
    handle_incomplete_single_underscore_italic,
};
use crate::html_tag_handler::handle_incomplete_html_tag;
use crate::inline_code_handler::handle_incomplete_inline_code;
use crate::katex_handler::{handle_incomplete_block_katex, handle_incomplete_inline_katex};
use crate::link_image_handler::handle_incomplete_links_and_images;
use crate::setext_heading_handler::handle_incomplete_setext_heading;
use crate::single_tilde_handler::handle_single_tilde_escape;
use crate::strikethrough_handler::handle_incomplete_strikethrough;

/// Determines how incomplete links are rendered while text is streaming.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LinkMode {
    /// Use the `streamdown:incomplete-link` placeholder URL.
    #[default]
    Protocol,
    /// Remove link markup and keep only the link text.
    TextOnly,
}

impl fmt::Display for LinkMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Protocol => formatter.write_str("protocol"),
            Self::TextOnly => formatter.write_str("text-only"),
        }
    }
}

/// A custom transformation in the remend handler pipeline.
pub trait RemendHandler {
    /// A stable identifier for this handler.
    fn name(&self) -> &str;

    /// The handler priority. Lower values run first.
    fn priority(&self) -> i32 {
        100
    }

    /// Transform the current Markdown text.
    fn handle(&self, text: &str) -> String;
}

/// Adapter that turns a closure into a [`RemendHandler`].
pub struct FnRemendHandler<F> {
    name: String,
    priority: i32,
    handle: F,
}

impl<F> FnRemendHandler<F> {
    /// Creates a handler with the default priority of `100`.
    pub fn new(name: impl Into<String>, handle: F) -> Self {
        Self {
            name: name.into(),
            priority: 100,
            handle,
        }
    }

    /// Creates a handler with an explicit priority.
    pub fn with_priority(name: impl Into<String>, priority: i32, handle: F) -> Self {
        Self {
            name: name.into(),
            priority,
            handle,
        }
    }
}

impl<F> RemendHandler for FnRemendHandler<F>
where
    F: Fn(&str) -> String,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn priority(&self) -> i32 {
        self.priority
    }

    fn handle(&self, text: &str) -> String {
        (self.handle)(text)
    }
}

/// Configuration for the remend pipeline.
pub struct RemendOptions {
    /// Complete bold formatting.
    pub bold: bool,
    /// Complete bold-italic formatting.
    pub bold_italic: bool,
    /// Escape comparison operators in list items.
    pub comparison_operators: bool,
    /// Custom handlers appended to the built-in pipeline.
    pub handlers: Vec<Box<dyn RemendHandler>>,
    /// Strip incomplete HTML tags at the end of streamed text.
    pub html_tags: bool,
    /// Remove incomplete images.
    pub images: bool,
    /// Complete inline code formatting.
    pub inline_code: bool,
    /// Complete inline KaTeX. Disabled by default because `$` is ambiguous.
    pub inline_katex: bool,
    /// Complete italic formatting.
    pub italic: bool,
    /// Complete block KaTeX formatting.
    pub katex: bool,
    /// How incomplete links are represented.
    pub link_mode: LinkMode,
    /// Complete incomplete links.
    pub links: bool,
    /// Protect incomplete setext headings from being misinterpreted.
    pub setext_headings: bool,
    /// Escape a single tilde between word characters.
    pub single_tilde: bool,
    /// Complete strikethrough formatting.
    pub strikethrough: bool,
}

impl Default for RemendOptions {
    fn default() -> Self {
        Self {
            bold: true,
            bold_italic: true,
            comparison_operators: true,
            handlers: Vec::new(),
            html_tags: true,
            images: true,
            inline_code: true,
            inline_katex: false,
            italic: true,
            katex: true,
            link_mode: LinkMode::Protocol,
            links: true,
            setext_headings: true,
            single_tilde: true,
            strikethrough: true,
        }
    }
}

impl RemendOptions {
    /// Creates the default configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a custom handler and returns the updated options.
    pub fn with_handler<H>(mut self, handler: H) -> Self
    where
        H: RemendHandler + 'static,
    {
        self.handlers.push(Box::new(handler));
        self
    }

    /// Adds a custom handler to this configuration.
    pub fn add_handler<H>(&mut self, handler: H)
    where
        H: RemendHandler + 'static,
    {
        self.handlers.push(Box::new(handler));
    }
}

const SINGLE_TILDE_PRIORITY: i32 = 0;
const COMPARISON_OPERATORS_PRIORITY: i32 = 5;
const HTML_TAGS_PRIORITY: i32 = 10;
const SETEXT_HEADINGS_PRIORITY: i32 = 15;
const LINKS_PRIORITY: i32 = 20;
const BOLD_ITALIC_PRIORITY: i32 = 30;
const BOLD_PRIORITY: i32 = 35;
const ITALIC_DOUBLE_UNDERSCORE_PRIORITY: i32 = 40;
const ITALIC_SINGLE_ASTERISK_PRIORITY: i32 = 41;
const ITALIC_SINGLE_UNDERSCORE_PRIORITY: i32 = 42;
const INLINE_CODE_PRIORITY: i32 = 50;
const STRIKETHROUGH_PRIORITY: i32 = 60;
const KATEX_PRIORITY: i32 = 70;
const INLINE_KATEX_PRIORITY: i32 = 75;

enum HandlerAction<'a> {
    Builtin(fn(&str) -> String),
    Links(LinkMode),
    Custom(&'a dyn RemendHandler),
}

impl HandlerAction<'_> {
    fn run(&self, text: &str) -> String {
        match self {
            Self::Builtin(handler) => handler(text),
            Self::Links(link_mode) => handle_incomplete_links_and_images(text, *link_mode),
            Self::Custom(handler) => handler.handle(text),
        }
    }
}

struct HandlerEntry<'a> {
    priority: i32,
    action: HandlerAction<'a>,
    early_return_on_incomplete_link: bool,
}

/// Completes incomplete Markdown syntax using the default configuration.
pub fn remend(text: &str) -> String {
    remend_with_options(text, &RemendOptions::default())
}

/// Completes incomplete Markdown syntax using explicit options.
pub fn remend_with_options(text: &str, options: &RemendOptions) -> String {
    let mut result = if text.ends_with(' ') && !text.ends_with("  ") {
        text[..text.len() - 1].to_owned()
    } else {
        text.to_owned()
    };

    let mut handlers = enabled_builtin_handlers(options);
    handlers.extend(options.handlers.iter().map(|handler| HandlerEntry {
        priority: handler.priority(),
        action: HandlerAction::Custom(handler.as_ref()),
        early_return_on_incomplete_link: false,
    }));
    handlers.sort_by_key(|entry| entry.priority);

    for entry in handlers {
        result = entry.action.run(&result);
        if entry.early_return_on_incomplete_link
            && result.ends_with("](streamdown:incomplete-link)")
        {
            return result;
        }
    }
    result
}

fn enabled_builtin_handlers(options: &RemendOptions) -> Vec<HandlerEntry<'static>> {
    let mut handlers = Vec::new();
    if options.single_tilde {
        handlers.push(builtin(SINGLE_TILDE_PRIORITY, handle_single_tilde_escape));
    }
    if options.comparison_operators {
        handlers.push(builtin(
            COMPARISON_OPERATORS_PRIORITY,
            handle_comparison_operators,
        ));
    }
    if options.html_tags {
        handlers.push(builtin(HTML_TAGS_PRIORITY, handle_incomplete_html_tag));
    }
    if options.setext_headings {
        handlers.push(builtin(
            SETEXT_HEADINGS_PRIORITY,
            handle_incomplete_setext_heading,
        ));
    }
    if options.links || options.images {
        handlers.push(HandlerEntry {
            priority: LINKS_PRIORITY,
            action: HandlerAction::Links(options.link_mode),
            early_return_on_incomplete_link: options.link_mode == LinkMode::Protocol,
        });
    }
    if options.bold_italic {
        handlers.push(builtin(BOLD_ITALIC_PRIORITY, handle_incomplete_bold_italic));
    }
    if options.bold {
        handlers.push(builtin(BOLD_PRIORITY, handle_incomplete_bold));
    }
    if options.italic {
        handlers.push(builtin(
            ITALIC_DOUBLE_UNDERSCORE_PRIORITY,
            handle_incomplete_double_underscore_italic,
        ));
        handlers.push(builtin(
            ITALIC_SINGLE_ASTERISK_PRIORITY,
            handle_incomplete_single_asterisk_italic,
        ));
        handlers.push(builtin(
            ITALIC_SINGLE_UNDERSCORE_PRIORITY,
            handle_incomplete_single_underscore_italic,
        ));
    }
    if options.inline_code {
        handlers.push(builtin(INLINE_CODE_PRIORITY, handle_incomplete_inline_code));
    }
    if options.strikethrough {
        handlers.push(builtin(
            STRIKETHROUGH_PRIORITY,
            handle_incomplete_strikethrough,
        ));
    }
    if options.katex {
        handlers.push(builtin(KATEX_PRIORITY, handle_incomplete_block_katex));
    }
    if options.inline_katex {
        handlers.push(builtin(
            INLINE_KATEX_PRIORITY,
            handle_incomplete_inline_katex,
        ));
    }
    handlers
}

fn builtin(priority: i32, handler: fn(&str) -> String) -> HandlerEntry<'static> {
    HandlerEntry {
        priority,
        action: HandlerAction::Builtin(handler),
        early_return_on_incomplete_link: false,
    }
}
