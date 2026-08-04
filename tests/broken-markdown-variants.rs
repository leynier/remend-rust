use remend::remend;

#[test]
fn translated_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("**bold*", "**bold**"),
        ("~~strike~", "~~strike~~"),
        ("$$formula$", "$$formula$$"),
        ("> * list with **bold", "> * list with **bold**"),
        ("> ~~struck text", "> ~~struck text~~"),
        ("- [ ] **bold task", "- [ ] **bold task**"),
        ("- [ ] *italic task", "- [ ] *italic task*"),
        ("- [ ] `code task", "- [ ] `code task`"),
        ("| **bold | next |", "| **bold | next |**"),
        ("| `code | next |", "| `code | next |`"),
        ("text <script>alert('", "text <script>alert('"),
        ("text <div class=\"test", "text"),
        ("text <br>", "text <br>"),
        ("text <!-- comment -->", "text <!-- comment -->"),
        ("$$\\frac{x}{y", "$$\\frac{x}{y$$"),
        ("$$\\begin{matrix} a", "$$\\begin{matrix} a$$"),
        ("**bold** then **more", "**bold** then **more**"),
        ("`code` then `more", "`code` then `more`"),
        ("*first* and *second", "*first* and *second*"),
        ("paragraph1\n\n**bold", "paragraph1\n\n**bold**"),
        ("line1\n\n*italic text", "line1\n\n*italic text*"),
        ("text\n\n\n**bold", "text\n\n\n**bold**"),
        ("text\n\n`code", "text\n\n`code`"),
        ("*日本語", "*日本語*"),
        ("**Hello 世界", "**Hello 世界**"),
        ("# Heading\n**bold", "# Heading\n**bold**"),
        ("> quote\n**bold", "> quote\n**bold**"),
        ("**bold\twith\ttabs", "**bold\twith\ttabs**"),
        ("**bold\r\nwith CRLF", "**bold\r\nwith CRLF**"),
        ("\n\n\n**bold", "\n\n\n**bold**"),
        ("text ", "text"),
        ("text  ", "text  "),
        ("**bold ", "**bold**"),
        ("## Important *note", "## Important *note*"),
        ("See ![diagram](http://example.com/img", "See "),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}

#[test]
fn upstream_literal_cases() {
    let cases: &[(&str, &str)] = &[
        ("**bold*", "**bold**"),
        ("~~strike~", "~~strike~~"),
        ("$$formula$", "$$formula$$"),
        ("> > **deeply nested bold", "> > **deeply nested bold**"),
        ("> * list with **bold", "> * list with **bold**"),
        (
            "> > > triple nested *italic",
            "> > > triple nested *italic*",
        ),
        ("> ~~struck text", "> ~~struck text~~"),
        ("- [ ] **bold task", "- [ ] **bold task**"),
        ("- [x] completed ~~struck~~", "- [x] completed ~~struck~~"),
        ("- [ ] *italic task", "- [ ] *italic task*"),
        ("- [ ] `code task", "- [ ] `code task`"),
        ("| **bold | next |", "| **bold | next |**"),
        ("| `code | next |", "| `code | next |`"),
        (
            "text <!-- incomplete comment",
            "text <!-- incomplete comment",
        ),
        ("text <script>alert('", "text <script>alert('"),
        ("text <div class=\"test", "text"),
        ("text <br>", "text <br>"),
        ("text <!-- comment -->", "text <!-- comment -->"),
        ("$$\\frac{x}{y", "$$\\frac{x}{y$$"),
        ("$$\\begin{matrix} a", "$$\\begin{matrix} a$$"),
        ("$$\n\\sum_{i=0}^{n} x_i", "$$\n\\sum_{i=0}^{n} x_i\n$$"),
        ("**bold** then **more", "**bold** then **more**"),
        ("`code` then `more", "`code` then `more`"),
        ("~~done~~ and ~~undone", "~~done~~ and ~~undone~~"),
        ("*first* and *second", "*first* and *second*"),
        ("***first*** and ***second", "***first*** and ***second***"),
        ("paragraph1\n\n**bold", "paragraph1\n\n**bold**"),
        ("line1\n\n*italic text", "line1\n\n*italic text*"),
        ("text\n\n\n**bold", "text\n\n\n**bold**"),
        ("text\n\n`code", "text\n\n`code`"),
        ("**中文粗体", "**中文粗体**"),
        ("*日本語", "*日本語*"),
        ("`한국어 코드", "`한국어 코드`"),
        ("~~🎉 celebration", "~~🎉 celebration~~"),
        ("**Hello 世界", "**Hello 世界**"),
        ("---\n**bold after rule", "---\n**bold after rule**"),
        ("# Heading\n**bold", "# Heading\n**bold**"),
        ("> quote\n**bold", "> quote\n**bold**"),
        ("```\ncode\n```\n*italic", "```\ncode\n```\n*italic*"),
        ("    *asterisks in indented", "    *asterisks in indented*"),
        ("    **bold in indented", "    **bold in indented**"),
        ("```\ncode\n```\n**bold", "```\ncode\n```\n**bold**"),
        ("```\nblock\n```\n`inline", "```\nblock\n```\n`inline`"),
        ("**bold\twith\ttabs", "**bold\twith\ttabs**"),
        ("**bold\r\nwith CRLF", "**bold\r\nwith CRLF**"),
        ("\n\n\n**bold", "\n\n\n**bold**"),
        ("text ", "text"),
        ("text  ", "text  "),
        ("**bold ", "**bold**"),
        (
            "1. First\n2. **Second item with bold",
            "1. First\n2. **Second item with bold**",
        ),
        (
            "The function `getData` returns a **Promise",
            "The function `getData` returns a **Promise**",
        ),
        (
            "Check the [documentation",
            "Check the [documentation](streamdown:incomplete-link)",
        ),
        (
            "- Use `map` to transform\n- Use `filter",
            "- Use `map` to transform\n- Use `filter`",
        ),
        ("## Important *note", "## Important *note*"),
        (
            "Here's the diagram:\n\n![architecture",
            "Here's the diagram:\n\n",
        ),
        ("See ![diagram](http://example.com/img", "See "),
        (
            "[click here](https://example.com) for **more",
            "[click here](https://example.com) for **more**",
        ),
    ];

    for (input, expected) in cases {
        assert_eq!(remend(input), *expected, "input: {input:?}");
    }
}
