use remend::{
    FnRemendHandler, LinkMode, RemendHandler, RemendOptions, count_single_asterisks,
    count_single_underscores, count_triple_asterisks, find_matching_closing_bracket,
    find_matching_opening_bracket, is_horizontal_rule, is_within_code_block, is_within_html_tag,
    is_within_link_or_image_url, is_within_math_block, is_word_char, remend, remend_with_options,
};

#[test]
fn options_disable_individual_handlers() {
    let options = RemendOptions {
        bold: false,
        ..Default::default()
    };
    assert_eq!(remend_with_options("**bold", &options), "**bold");

    let options = RemendOptions {
        italic: false,
        ..Default::default()
    };
    assert_eq!(remend_with_options("*italic", &options), "*italic");

    let options = RemendOptions {
        inline_code: false,
        ..Default::default()
    };
    assert_eq!(remend_with_options("`code", &options), "`code");

    let options = RemendOptions {
        strikethrough: false,
        ..Default::default()
    };
    assert_eq!(remend_with_options("~~strike", &options), "~~strike");

    let options = RemendOptions {
        katex: false,
        ..Default::default()
    };
    assert_eq!(remend_with_options("$$formula", &options), "$$formula");

    let options = RemendOptions {
        inline_katex: true,
        ..Default::default()
    };
    assert_eq!(remend_with_options("$formula", &options), "$formula$");

    let options = RemendOptions {
        single_tilde: false,
        ..Default::default()
    };
    assert_eq!(remend_with_options("20~25", &options), "20~25");

    let options = RemendOptions {
        comparison_operators: false,
        ..Default::default()
    };
    assert_eq!(remend_with_options("- > 25", &options), "- > 25");

    let options = RemendOptions {
        html_tags: false,
        ..Default::default()
    };
    assert_eq!(
        remend_with_options("text <custom", &options),
        "text <custom"
    );

    let options = RemendOptions {
        setext_headings: false,
        ..Default::default()
    };
    assert_eq!(remend_with_options("title\n-", &options), "title\n-");
}

#[test]
fn link_modes_and_image_option_follow_the_public_contract() {
    let options = RemendOptions {
        link_mode: LinkMode::TextOnly,
        ..Default::default()
    };
    assert_eq!(
        remend_with_options("Check [this link](https://example", &options),
        "Check this link"
    );
    assert_eq!(
        remend_with_options("Text [partial", &options),
        "Text partial"
    );

    let options = RemendOptions {
        links: false,
        images: true,
        ..Default::default()
    };
    assert_eq!(
        remend_with_options("[partial", &options),
        "[partial](streamdown:incomplete-link)"
    );

    let options = RemendOptions {
        links: false,
        images: false,
        ..Default::default()
    };
    assert_eq!(remend_with_options("[partial", &options), "[partial");
}

#[test]
fn custom_handlers_support_traits_closures_and_priorities() {
    let bang = FnRemendHandler::new("bang", |text: &str| format!("{text}!"));
    let options = RemendOptions::default().with_handler(bang);
    assert_eq!(remend_with_options("**bold", &options), "**bold**!");

    let high = FnRemendHandler::with_priority("high", 20, |text: &str| format!("{text}H"));
    let low = FnRemendHandler::with_priority("low", 80, |text: &str| format!("{text}L"));
    let options = RemendOptions::default()
        .with_handler(low)
        .with_handler(high);
    assert_eq!(remend_with_options("text", &options), "textHL");

    struct Prefix;
    impl RemendHandler for Prefix {
        fn name(&self) -> &str {
            "prefix"
        }

        fn priority(&self) -> i32 {
            90
        }

        fn handle(&self, text: &str) -> String {
            format!("prefix:{text}")
        }
    }

    let mut options = RemendOptions::default();
    options.add_handler(Prefix);
    assert_eq!(remend_with_options("text", &options), "prefix:text");
}

#[test]
fn custom_handlers_preserve_stable_order_and_can_replace_builtins() {
    let first = FnRemendHandler::new("replace-a", |text: &str| text.replace('a', "b"));
    let second = FnRemendHandler::new("replace-b", |text: &str| text.replace('b', "c"));
    let options = RemendOptions::default()
        .with_handler(first)
        .with_handler(second);
    assert_eq!(remend_with_options("aaa", &options), "ccc");

    let suffix = FnRemendHandler::new("suffix", |text: &str| format!("{text}!"));
    let options = RemendOptions {
        bold: false,
        ..Default::default()
    }
    .with_handler(suffix);
    assert_eq!(remend_with_options("**bold", &options), "**bold!");

    let uppercase = FnRemendHandler::new("uppercase", |text: &str| text.to_uppercase());
    let options = RemendOptions {
        bold: false,
        bold_italic: false,
        comparison_operators: false,
        html_tags: false,
        images: false,
        inline_code: false,
        italic: false,
        katex: false,
        links: false,
        setext_headings: false,
        single_tilde: false,
        strikethrough: false,
        ..Default::default()
    }
    .with_handler(uppercase);
    assert_eq!(remend_with_options("hello", &options), "HELLO");

    assert_eq!(remend("**bold"), "**bold**");
}

#[test]
fn public_context_utilities_use_safe_byte_offsets() {
    assert!(is_word_char('a'));
    assert!(is_word_char('é'));
    assert!(is_word_char('_'));
    assert!(!is_word_char('-'));

    assert!(is_within_code_block("before ```code", 14));
    assert!(!is_within_code_block("before ```code```", 17));
    assert!(is_within_math_block("$$x$y$$z", 5));
    assert!(is_within_link_or_image_url("[text](url)", 8));
    assert!(!is_within_link_or_image_url("[text](url) after", 13));
    assert!(is_within_html_tag("<div data_attr", 12));
    assert!(!is_within_html_tag("<div> text", 6));

    assert_eq!(
        find_matching_opening_bracket("[outer [inner] text]", 19),
        Some(0)
    );
    assert_eq!(find_matching_opening_bracket("some text]", 9), None);
    assert_eq!(
        find_matching_closing_bracket("[outer [inner] text]", 0),
        Some(19)
    );
    assert_eq!(find_matching_closing_bracket("[some text", 0), None);
    assert_eq!(find_matching_closing_bracket("é[text]", 2), Some(7));

    assert!(is_horizontal_rule("* * *", 0, '*'));
    assert!(is_horizontal_rule("*\t*\t*", 0, '*'));
    assert!(!is_horizontal_rule("text * * *", 5, '*'));

    assert_eq!(count_triple_asterisks("text***"), 1);
    assert_eq!(count_triple_asterisks("```\n***\n```"), 0);
    assert_eq!(count_single_asterisks("*italic*"), 2);
    assert_eq!(count_single_underscores("_italic_"), 2);
}

#[test]
fn handler_defaults_and_link_mode_are_stable() {
    assert_eq!(LinkMode::default(), LinkMode::Protocol);
    assert_eq!(LinkMode::Protocol.to_string(), "protocol");
    assert_eq!(LinkMode::TextOnly.to_string(), "text-only");

    let options = RemendOptions::new();
    assert!(options.bold);
    assert!(options.bold_italic);
    assert!(options.links);
    assert!(!options.inline_katex);
    assert_eq!(options.link_mode, LinkMode::Protocol);
}
