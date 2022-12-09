use error_chain::error_chain;

error_chain! {
  foreign_links {
      Io(std::io::Error);
      HttpRequest(reqwest::Error);
      Url(url::ParseError);
      Toml(toml::ser::Error);
      Cbor(serde_cbor::Error);
  }

  errors {
    CacheFailure(reason: String) {
      description("A cache operation failed")
      display("{}", reason)
    }
  }
  // skip_msg_variant
}
