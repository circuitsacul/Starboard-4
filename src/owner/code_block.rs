pub fn parse_code_blocks(content: &str) -> Vec<(String, String)> {
    let mut blocks = Vec::new();
    let mut meta = "".to_string();
    let mut current_block = Vec::new();
    let mut in_block = false;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("```") {
            in_block = !in_block;
            if !in_block && !current_block.is_empty() {
                blocks.push((
                    current_block.join("\n"),
                    meta,
                ));
                current_block.clear();
                meta = "".to_string();
            }
            continue;
        }

        if in_block {
            current_block.push(line.to_string());
        } else if line != "" {
            meta.push_str(line);
            meta.push_str("\n");
        }
    }

    blocks
}
