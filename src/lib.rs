pub fn culling(input: &str) -> String {
    let html: Vec<char> = input.chars().collect();
    let mut output: String = String::new();
    let mut tag = String::new();
    let html_length = html.len();
    let mut index = 0;
    let tags: Vec<&str> = vec![
        "<title>", "</title>", "<br>", "<br />", "<h1", "</h1>", "<h2", "</h2>", "<h3", "</h3>", "<h4",
        "</h4>", "<h5", "</h5>", "<h6", "</h6>", "<b>", "<b ", "</b>", "<i>", "<i ", "</i>", "<li>", "<li ",
        "</li>", "<ul", "</ul", "<ol>", "<ol ", "</ol", "<code", "</code", "<pre>", "<pre ", "</pre>",
        "<p>", "<p ", "</p>", "<a>", "<a ", "</a>"
    ];
    let number_of_tags = tags.len();

    while index < html_length {
        if html[index] == '<' {
            while html[index] != '>' {
                tag.push(html[index]);
                index += 1;
            }
            tag.push(html[index]);
            index += 1;
            if tag.starts_with("<script") {
                while !tag.ends_with("</script>") {
                    tag.push(html[index]);
                    index += 1;
                }
                tag = String::new();
                continue;
            }

            if tag.starts_with("<style") {
                while !tag.ends_with("</style>") {
                    tag.push(html[index]);
                    index += 1;
                }
                tag = String::new();
                continue;
            }
            for i in 0..number_of_tags {
                if tag.contains(tags[i]) {
                    output = format!("{}{}", output, tag.chars().collect::<String>());
                    break;
                }
            }

            tag = String::new();
        }
        output = format!("{}{}", output, html[index]);
        index += 1;
    }
    return output;
}
