
pub macro expand($span: ident, $source: expr) {
  crate::common::utils::get_utf8_slice($source.code, $span.start, $span.end).ok_or_else(|| crate::common::error::Error::new(crate::common::error:ErrorKind::UnexpectedEndOfInput, $self.cursor.source, $self.cursor.span()))
}
pub macro token($self: ident, $kind: ident) {
  super::token::Token {
    kind: super::token::TokenKind::$kind,
    span: $self.cursor.span(),
    source: $self.cursor.source
  }
}
pub macro product($self: ident, $producer: ident, $token_type: ident) {
  {
    
    $self.$producer()?;
    let token = token!($self, $token_type);
    $self.cursor.clear_span();
    Ok(token)
  }
}
pub macro single_product($self: ident, $tt: ident) {
  {
    $self.cursor.next();
    let token = token!($self, $tt);
    $self.cursor.clear_span();
    Ok(token)
  }
}