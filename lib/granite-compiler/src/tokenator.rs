use proc_macro2::Delimiter;
use proc_macro2_diagnostics::SpanDiagnosticExt;

/// This is a generic wrapper around a TokenTree which allows the proc macro
/// to extract the Diagnostic and return the correct set of tokens which highlights
/// the specific piece of syntax that caused the problem
pub struct TokenError {
    span: proc_macro2::Span,
    error: String,
}

impl TokenError {
    pub fn new(span: proc_macro2::Span, error: &str) -> TokenError {
        TokenError {
            span,
            error: error.to_string(),
        }
    }

    pub fn new_call_site(error: &str) -> TokenError {
        TokenError {
            span: proc_macro2::Span::call_site(),
            error: error.to_string(),
        }
    }

    /// To be used by the proc macro
    pub fn get_diagnostic(&self) -> proc_macro2_diagnostics::Diagnostic {
        // requires: use proc_macro2_diagnostics::SpanDiagnosticExt;
        self.span.error(&self.error)
    }

    /// To be used by build.rs where it needs to eprintln!() this but skip processing this item.
    /// If build.rs panics, then nothing else happens, so it's important it never panics.
    pub fn get_error_str(&self) -> &str {
        &self.error
    }

    /// To be used by testing rigs
    pub fn panic(&self) -> ! {
        panic!("{}", self.error);
    }
}

pub struct TokenIter {
    token_stream_iter: proc_macro2::token_stream::IntoIter,
    previous_tree: Option<proc_macro2::TokenTree>,
    current_tree: Option<proc_macro2::TokenTree>,
    current_token: Token,
}

//TODO: Evaluate how to rid this of uneeded clones
impl TokenIter {
    pub fn new(tokens: proc_macro2::TokenStream) -> TokenIter {
        let token_stream_iter = tokens.into_iter();
        TokenIter {
            token_stream_iter,
            previous_tree: None,
            current_tree: None,
            current_token: Token::Start,
        }
    }

    pub fn step(&mut self) {
        self.previous_tree = self.current_tree.take();
        self.current_tree = self.token_stream_iter.next();
        self.current_token = Token::from(self.current_tree.clone());
    }

    /// Consumes the inner token stream up to the ending character
    /// The ending character is consumed but not included in the returned token stream.
    /// If end is encountered, the token stream is returned sitting on the End
    pub fn take_token_stream_until_char_or_end(
        &mut self,
        ending_char: char,
    ) -> Result<proc_macro2::TokenStream, TokenError> {
        let mut tokens = proc_macro2::TokenStream::new();

        while let Some(current_tree) = &self.current_tree {
            if matches!(current_tree, proc_macro2::TokenTree::Punct(p) if p.as_char() == ending_char)
            {
                self.step();
                break;
            }
            tokens.extend(std::iter::once(current_tree.clone()));
            self.step();
        }

        Ok(tokens)
    }

    pub fn token(&self) -> &Token {
        &self.current_token
    }

    pub fn tree_ref(&self) -> Result<Option<&proc_macro2::TokenTree>, TokenError> {
        Ok(self.current_tree.as_ref())
    }

    /// Attempt to generate an error based on the current token, but if it is None, then use the previous token if there was one.
    /// Note: this is a bit of a hack to get the red underline to not use call_site unless totally necessary.
    pub fn error(&self, error: &str) -> TokenError {
        TokenError::new(
            match (&self.current_tree, &self.previous_tree) {
                (Some(token), _) => token.span(),
                (None, Some(token)) => token.span(),
                (None, None) => proc_macro2::Span::call_site(),
            },
            error,
        )
    }

    pub fn get_brace_group_iter(&mut self) -> Result<Self, TokenError> {
        match &self.current_token {
            Token::Group(group) if group.delimiter() == Delimiter::Brace => {
                Ok(Self::new(group.stream()))
            }
            _ => Err(self.error("expected `{` Group")),
        }
    }

    pub fn take_brace_group_iter(&mut self) -> Result<Self, TokenError> {
        let iter = self.get_brace_group_iter()?;
        self.step();
        Ok(iter)
    }

    pub fn take_bracket_group_iter(&mut self) -> Result<Self, TokenError> {
        match &self.current_token {
            Token::Group(group) if group.delimiter() == Delimiter::Bracket => {
                let iter = Self::new(group.stream());
                self.step();
                Ok(iter)
            }
            _ => Err(self.error("expected `[` Group")),
        }
    }

    pub fn get_ident(&mut self) -> Result<proc_macro2::Ident, TokenError> {
        match &self.current_token {
            Token::Ident(ident) => Ok(ident.to_owned()),
            _ => Err(self.error("expected Ident")),
        }
    }

    pub fn take_ident(&mut self) -> Result<proc_macro2::Ident, TokenError> {
        let ident = self.get_ident()?;
        self.step();
        Ok(ident)
    }

    pub fn get_ident_as_string(&mut self) -> Result<String, TokenError> {
        Ok(self.get_ident()?.to_string())
    }

    pub fn take_ident_as_string(&mut self) -> Result<String, TokenError> {
        let ident = self.get_ident_as_string()?;
        self.step();
        Ok(ident)
    }

    pub fn get_ident_match(&mut self, ident: &str) -> Result<(), TokenError> {
        match &self.current_token {
            Token::Ident(i) if i.to_string().as_str() == ident => Ok(()),
            _ => Err(self.error(&format!("expected `{}`", ident))),
        }
    }

    pub fn get_dash(&mut self) -> Result<char, TokenError> {
        match &self.current_token {
            Token::Dash => Ok('-'),
            _ => Err(self.error("expected `-`")),
        }
    }

    pub fn get_equals(&mut self) -> Result<char, TokenError> {
        match &self.current_token {
            Token::Equal => Ok('='),
            _ => Err(self.error("expected `=`")),
        }
    }

    pub fn take_equals(&mut self) -> Result<(), TokenError> {
        self.get_equals()?;
        self.step();
        Ok(())
    }

    pub fn get_less_than(&mut self) -> Result<char, TokenError> {
        match &self.current_token {
            Token::LessThan => Ok('<'),
            _ => Err(self.error("expected `<`")),
        }
    }

    pub fn get_greater_than(&mut self) -> Result<char, TokenError> {
        match &self.current_token {
            Token::GreaterThan => Ok('>'),
            _ => Err(self.error("expected `>`")),
        }
    }

    // get apostrophe
    pub fn get_apostrophe(&mut self) -> Result<char, TokenError> {
        match &self.current_token {
            Token::Apostrophe => Ok('\''),
            _ => Err(self.error("expected `'`")),
        }
    }

    pub fn get_colon(&mut self) -> Result<char, TokenError> {
        match &self.current_token {
            Token::Colon => Ok(':'),
            _ => Err(self.error("expected `:`")),
        }
    }

    pub fn take_colon(&mut self) -> Result<(), TokenError> {
        self.get_colon()?;
        self.step();
        Ok(())
    }

    pub fn get_comma(&mut self) -> Result<char, TokenError> {
        match &self.current_token {
            Token::Comma => Ok(','),
            _ => Err(self.error("expected `,`")),
        }
    }

    pub fn take_comma(&mut self) -> Result<(), TokenError> {
        self.get_comma()?;
        self.step();
        Ok(())
    }

    pub fn get_dollar_sign(&mut self) -> Result<char, TokenError> {
        match &self.current_token {
            Token::DollarSign => Ok('$'),
            _ => Err(self.error("expected `$`")),
        }
    }

    pub fn take_dollar_sign(&mut self) -> Result<(), TokenError> {
        match &self.current_token {
            Token::DollarSign => {
                self.step();
                Ok(())
            }
            _ => Err(self.error("expected `$`")),
        }
    }

    pub fn get_semicolon(&mut self) -> Result<char, TokenError> {
        match &self.current_token {
            Token::Semicolon => Ok(';'),
            _ => Err(self.error("expected `;`")),
        }
    }

    pub fn take_semicolon(&mut self) -> Result<(), TokenError> {
        self.get_semicolon()?;
        self.step();
        Ok(())
    }

    pub fn get_slash(&mut self) -> Result<char, TokenError> {
        match &self.current_token {
            Token::Slash => Ok('/'),
            _ => Err(self.error("expected `/`")),
        }
    }

    pub fn get_dash_dot_underscore_option(&mut self) -> Option<char> {
        match self.current_token {
            Token::Dash => Some('-'),
            Token::Dot => Some('.'),
            Token::Underscore => Some('_'),
            _ => None,
        }
    }

    pub fn get_end(&mut self) -> Result<(), TokenError> {
        match &self.current_token {
            Token::End => Ok(()),
            _ => Err(self.error("expected end")),
        }
    }

    pub fn is_question_mark(&mut self) -> bool {
        matches!(self.current_token, Token::QuestionMark)
    }

    pub fn is_comma(&mut self) -> bool {
        matches!(self.current_token, Token::Comma)
    }

    pub fn is_colon(&mut self) -> bool {
        matches!(self.current_token, Token::Colon)
    }

    pub fn is_mut(&mut self) -> bool {
        matches!(self.current_token, Token::Ident(ref i) if i.to_string().eq("mut"))
    }

    pub fn is_impl(&mut self) -> bool {
        matches!(self.current_token, Token::Ident(ref i) if i.to_string().eq("impl"))
    }

    pub fn is_end(&mut self) -> bool {
        matches!(self.current_token, Token::End)
    }

    /// this is expected to start on a < and end after the >
    /// The job of this function is to parse the `<crate::Foo, crate::Bar>` type segments of a signature
    pub fn take_less_than_paths_greater_than(&mut self) -> Result<Vec<String>, TokenError> {
        // take the <
        self.get_less_than()?;
        self.step();

        let mut paths = Vec::new();
        loop {
            let mut path = String::new();

            loop {
                // take first ident
                path.push_str(&self.get_ident_as_string()?);
                self.step();

                // if a colon, take ::, and loop to get then next ident
                if self.is_colon() {
                    self.step();
                    self.get_colon()?;
                    self.step();
                    path.push_str("::");
                } else {
                    break;
                }
            }

            paths.push(path);

            // now to identify if we are done or have another
            match self.token() {
                Token::Comma => {
                    self.step();
                    continue;
                }
                Token::GreaterThan => {
                    self.step();
                    break;
                }
                _ => {
                    return Err(self.error("expected `,` or `>`"));
                }
            }
        }

        Ok(paths)
    }

    // Other methods as needed...
}

#[derive(Debug, Clone)]
pub enum Token {
    Ampersand,
    Asterisk,
    Apostrophe,
    Colon,
    Comma,
    Dash,
    DollarSign,
    Dot,
    End,
    Equal,
    GreaterThan,
    LessThan,
    Pipe,
    Plus,
    QuestionMark,
    Semicolon,
    Slash,
    Start,
    Underscore,
    Ident(proc_macro2::Ident),
    Punct(proc_macro2::Punct),
    Literal(proc_macro2::Literal),
    Group(proc_macro2::Group),
}

impl From<Option<proc_macro2::TokenTree>> for Token {
    fn from(token: Option<proc_macro2::TokenTree>) -> Token {
        match token {
            Some(proc_macro2::TokenTree::Ident(ident)) => Token::Ident(ident),
            Some(proc_macro2::TokenTree::Punct(punct)) => match punct.as_char() {
                '&' => Token::Ampersand,
                '\'' => Token::Apostrophe,
                '*' => Token::Asterisk,
                ',' => Token::Comma,
                ':' => Token::Colon,
                '-' => Token::Dash,
                '$' => Token::DollarSign,
                '.' => Token::Dot,
                '=' => Token::Equal,
                '>' => Token::GreaterThan,
                '<' => Token::LessThan,
                '|' => Token::Pipe,
                '+' => Token::Plus,
                '?' => Token::QuestionMark,
                ';' => Token::Semicolon,
                '/' => Token::Slash,
                '_' => Token::Underscore,
                _ => Token::Punct(punct),
            },
            Some(proc_macro2::TokenTree::Literal(literal)) => Token::Literal(literal),
            Some(proc_macro2::TokenTree::Group(group)) => Token::Group(group),
            None => Token::End,
        }
    }
}
