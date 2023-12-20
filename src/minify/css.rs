const SPECIAL_CHARS: [char; 8] = ['{', '}', ':', ';', ' ', '\n', '!', '>'];

// Taken from https://github.com/amgarrett09/rust-css-minifier/blob/master/src/minify/mod.rs.
pub fn css(input: &str) -> String {
    let mut last_char: Vec<char> = " ".chars().collect();
    let mut output = Vec::new();

    let mut inside_comment = false;

    for c in input.chars() {
        // We're in a comment if we find '/*'
        if !inside_comment && c == '*' && last_char[0] == '/' {
            inside_comment = true;
            output.pop();
        }

        // We're no longer in a comment if we find '*/'
        if inside_comment && c == '/' && last_char[0] == '*' {
            inside_comment = false;
        }

        // We should NOT add a char to the output if:
        // 1) It's a line break, OR
        // 2) The char is a space AND the last char scanned was one of our
        // special cases OR
        // 3) We're inside a comment
        // should_add_char is the negation of that
        if !(c == '\n' || (c == ' ' && SPECIAL_CHARS.contains(&last_char[0])) || inside_comment) {
            // Remove last char (and don't put it back) if it's a space before
            // a special character, or if it's a semicolon before an ending brace
            if let Some(last) = output.pop() {
                if (!SPECIAL_CHARS.contains(&c) || last != ' ') && (c != '}' || last != ';') {
                    output.push(last);
                }
            }

            output.push(c);
        }

        last_char[0] = c;
    }

    output.iter().collect()
}
