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
    
    let ignore_tags: Vec<&str> = vec![
        "<script", "<style", "<iframe", "<noscript", "<symbol",
    ];
    
    while index < html_length {
        if html[index] == '<' {
            let tag_start = index;
            let mut tag = String::new();
            while index < html_length && html[index] != '>' {
                tag.push(html[index]);
                index += 1;
            }
            
            // '>'が見つからない場合はループを終了
            if index >= html_length {
                break;
            }
            
            tag.push(html[index]); // '>'を追加
            index += 1;
            
            let mut should_ignore = false;
            for ignore_tag in &ignore_tags {
                if tag.to_lowercase().starts_with(ignore_tag) {
                    should_ignore = true;
                    let end_tag = match *ignore_tag {
                        "<script" => "</script>",
                        "<style" => "</style>",
                        "<iframe" => "</iframe>",
                        "<noscript" => "</noscript>",
                        "<symbol" => "</symbol>",
                        _ => "",
                    };
                    
                    let end_tag_chars: Vec<char> = end_tag.chars().collect();
                    let end_tag_len = end_tag_chars.len();
                    
                    while index < html_length {
                        // 終了タグの開始位置を探す
                        if index + end_tag_len <= html_length {
                            let mut matches = true;
                            for i in 0..end_tag_len {
                                if html[index + i].to_lowercase().next().unwrap() != end_tag_chars[i] {
                                    matches = false;
                                    break;
                                }
                            }
                            if matches {
                                index += end_tag_len;
                                break;
                            }
                        }
                        index += 1;
                    }
                    break;
                }
            }
            
            if !should_ignore {
                let mut is_allowed_tag = false;
                for allowed_tag in &tags {
                    if tag.to_lowercase().contains(allowed_tag) {
                        is_allowed_tag = true;
                        break;
                    }
                }
                
                if is_allowed_tag {
                    output.push_str(&tag);
                }
            }
        } else {
            output.push(html[index]);
            index += 1;
        }
    }
    
    output
}
