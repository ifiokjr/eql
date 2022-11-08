use edgeql_parser::tokenizer::SpannedToken;
use edgeql_parser::tokenizer::TokenStream;

pub fn tokenize(input: &str) -> Vec<SpannedToken> {
  let mut token_stream = TokenStream::new(input);
  let mut tokens = Vec::new();

  for token_result in &mut token_stream {
    match token_result {
      Ok(token) => tokens.push(token),
      Err(_error) => {
        println!("Error: {:?}", _error);
        // handle errors here
      }
    }
  }

  tokens
}

#[cfg(test)]
mod tests {
  use crate::parser::tokenize;

  #[test]
  fn simple_module_tokens() {
    assert_eq!(4, 4);
    let tokens = tokenize("module default {}");
    insta::assert_debug_snapshot!(tokens);
  }
}
