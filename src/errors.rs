use error_chain::error_chain;

error_chain! {
  foreign_links {
      Io(std::io::Error);
      HttpRequest(reqwest::Error);
      Url(url::ParseError);
  }
  // skip_msg_variant
}
