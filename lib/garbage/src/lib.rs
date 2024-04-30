/// This is going to use the `?` to unwrap the result, so it has to be in a function that
/// can handle this behavior.
#[macro_export]
macro_rules! JE {
    ($data:expr) => {
        //TODO: Unwrap is bad, but this is a macro, so we can't use the ? operator... how to fix?
        serde_json::to_string(&$data).unwrap()
    };
}

#[macro_export]
macro_rules! HS {
    ($data:expr) => {
        html_escape::encode_text(&$data).to_string()
    };
}

#[macro_export]
macro_rules! QA {
    ($data:expr) => {
        format!("\"{}\"", html_escape::encode_text(&$data))
    };
}

/// Runtime concatenation of string expressions.
/// Pass as many as needed to this macro to return a single concatenated string
#[macro_export]
macro_rules! CN {
    ($($arg:expr),*) => {{
        let mut result = String::new();
        $(
            result += $arg.as_ref();
        )*
        result
    }};
}

#[macro_export]
macro_rules! JN {
    ($iter:expr, $func:expr) => {{
        $iter.into_iter().map($func).collect::<String>()
    }};
}

/// Runtime concatenation of string expressions with runtime stripping of common prefix.
#[macro_export]
macro_rules! CNSL {
    ($($arg:expr),*) => {{
        let mut result = String::new();
        $(
            result += $arg.as_ref();
        )*
        let mut lines: Vec<_> = result.lines().collect();


        if ! lines.is_empty() {
            if lines.first().unwrap().trim() != "" {
                panic!("There must be only whitespace for the first line.");
            }
            lines.remove(0);
        }

        if ! lines.is_empty() {
            if lines.last().unwrap().trim() != "" {
                panic!("There must be only whitespace after last newline.");
            }

            // change the last line to just "\n"
            *lines.last_mut().unwrap() = "";
        }

        let mut common_prefix = None;
        for line in &lines {
            if !line.trim().is_empty() {
                common_prefix = Some(line.chars().take_while(|c| c.is_whitespace()).collect::<String>());
                break;
            }
        }
        let common_prefix = match common_prefix {
            Some(prefix) => prefix,
            None => String::new(),
        };
        let mut new_lines = Vec::new();
        for line in lines {
            // Strip if possible, otherwise ignore
            match line.strip_prefix(&common_prefix) {
                Some(line) => new_lines.push(line),
                None => new_lines.push(line),
            }
        }
        new_lines.join("\n")
    }};
}

/// Static Strip Lines
/// At compile time this will strip consistent leading whitespace off of a block of text based on the
/// Whitespace found before the first non-whitespace line.  Basically dedent.
/// To be used with static strings at compile time.
/// Consider CNSL for runtime strings with concatenation.
#[macro_export]
macro_rules! STSL {
    ($text:expr) => {{
        let text: &str = $text;
        let mut lines = text.lines().collect::<Vec<&str>>();

        if lines.is_empty() {
            "".to_string()
        } else {
            if lines[0] == "" {
                lines.remove(0);
            }

            if lines.is_empty() {
                "".to_string()
            } else {
                if lines.last().unwrap().trim() != "" {
                    panic!("There must be only whitespace after last newline.");
                }

                let prefix_line = lines.iter().find(|&line| !line.trim().is_empty());
                let strip_prefix = match prefix_line {
                    Some(line) => line
                        .chars()
                        .take_while(|c| c.is_whitespace())
                        .collect::<String>(),
                    None => "".to_string(),
                };

                lines
                    .iter()
                    .map(|line| {
                        if line.starts_with(&strip_prefix) {
                            line.strip_prefix(&strip_prefix).unwrap_or("").to_string()
                        } else {
                            line.to_string()
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
                    + "\n"
            }
        }
    }};
}

pub type AsyncCallback<T, E> =
    std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send>>;

#[cfg(test)]
pub mod test {

    #[test]
    pub fn test_cnsl_beginning_end_whitespace() {
        #[rustfmt::skip]
        let result = super::CNSL!(r#"
            #!/bin/bash
            echo "Hello World"
        "#);
        assert_eq!(result, "#!/bin/bash\necho \"Hello World\"\n");
    }

    #[test]
    pub fn test_cnsl_empty() {
        let result = super::CNSL!(r#""#);
        assert_eq!(result, "");
    }
}
