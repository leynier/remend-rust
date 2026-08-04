use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        (" ", ""),
        ("__content_", "__content__"),
        ("_text**", "_text**_"),
        ("```\n***bold", "```\n***bold"),
        ("\n=", "\n="),
        ("\n==", "\n=="),
        ("a~~b~~text", "a~~b~~text"),
        ("a~~b~~c~", "a~~b~~c~"),
        ("](partial", "](partial"),
        ("[link](url) _word", "[link](url) _word_"),
        ("func(_arg", "func(_arg_"),
        ("div> _text", "div> _text_"),
        ("3<5 _text", "3<5 _text_"),
        ("<div>\n_text", "<div>\n_text_"),
        ("[link](a_b) _word", "[link](a_b) _word_"),
        ("```\n__content_", "```\n__content_"),
        ("__a__ __b__content_", "__a__ __b__content_"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        (" ", ""),
        ("__content_", "__content__"),
        ("_text**", "_text**_"),
        ("```\n***bold", "```\n***bold"),
        ("```\n***\n```\n***text", "```\n***\n```\n***text***"),
        ("```\n_code\n```\n_text", "```\n_code\n```\n_text_"),
        ("\n=", "\n="),
        ("\n==", "\n=="),
        ("a~~b~~text", "a~~b~~text"),
        ("a~~b~~c~", "a~~b~~c~"),
        ("```\n__code\n```\n__text", "```\n__code\n```\n__text__"),
        ("](partial", "](partial"),
        ("[link](url) _word", "[link](url) _word_"),
        ("func(_arg", "func(_arg_"),
        ("div> _text", "div> _text_"),
        ("3<5 _text", "3<5 _text_"),
        ("<div>\n_text", "<div>\n_text_"),
        ("[link](a_b) _word", "[link](a_b) _word_"),
        ("```\n__content_", "```\n__content_"),
        ("__a__ __b__content_", "__a__ __b__content_"),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
