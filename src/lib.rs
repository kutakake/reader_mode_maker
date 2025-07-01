pub fn culling(input: &str) -> String {
    let html: Vec<char> = input.chars().collect();
    let mut output: String = String::new();
    let html_length = html.len();
    let mut index = 0;
    let tags: Vec<&str> = vec![
        "<title>", "</title>", "<br>", "<br />", "<h1", "</h1>", "<h2", "</h2>", "<h3", "</h3>",
        "<h4", "</h4>", "<h5", "</h5>", "<h6", "</h6>", "<b>", "<b ", "</b>", "<i>", "<i ", "</i>",
        "<li>", "<li ", "</li>", "<ul", "</ul", "<ol>", "<ol ", "</ol", "<code", "</code", "<pre>",
        "<pre ", "</pre>", "<p>", "<p ", "</p>", "<a>", "<a ", "</a>",
    ];
    let number_of_tags = tags.len();

    let ignore_list: Vec<&str> = vec![
        "<script", "<style", "<iframe", "<noscript", "<symbol",
    ];

    let ignore_end_list: Vec<&str> = vec![
        "</script>", "</style>", "</iframe>", "</noscript>", "</symbol>",
    ];

    let mut ignored = false;

    let number_of_ignore_list = ignore_list.len();

    while index < html_length {
        if html[index] == '<' {
            let mut tag = String::new();
            while html[index] != '>' {
                tag.push(html[index]);
                index += 1;
            }
            tag.push(html[index]);
            index += 1;


            for i in 0..number_of_ignore_list {
                if tag.to_lowercase().starts_with(ignore_list[i]) {
                    while !tag.to_lowercase().ends_with(ignore_end_list[i]) && index < html_length {
                        tag.push(html[index]);
                        index += 1;
                    }
                    ignored = true;
                    break;
                }
            }

            if ignored {
                ignored = false;
                continue;
            }

            for i in 0..number_of_tags {
                if tag.to_lowercase().contains(tags[i]) {
                    output = format!("{}{}", output, tag.chars().collect::<String>());
                    break;
                }
            }
        }
        output = format!("{}{}", output, html[index]);
        index += 1;
    }
    return output;
}
