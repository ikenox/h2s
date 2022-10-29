//! Implementations of `FromHtml` trait

use crate::from_text::FromText;
use crate::text_extractor::impls::AttributeNotFound;
use crate::Error;
use crate::{FromHtml, HtmlNode};
use std::fmt::{Debug, Display};


impl<S: FromText> FromHtml for S
where
    S::Error: Debug + Display + 'static,
{
    type Args = ExtractionType;
    type Error = FromHtmlTextError<AttributeNotFound, S::Error>;

    fn from_html<N: HtmlNode>(source: &N, args: &Self::Args) -> Result<Self, Self::Error> {
        let txt = args
            .extract(source)
            .map_err(FromHtmlTextError::ExtractionFailed)?;
        S::from_text(&txt).map_err(FromHtmlTextError::TextParseError)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StructFieldError<A: Error, B: Error> {
    pub selector: Option<String>,
    pub field_name: String,
    pub error: StructErrorCause<A, B>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StructErrorCause<A: Error, B: Error> {
    StructureUnmatched(A),
    ParseError(B),
}

pub enum ExtractionType {
    Text,
    Attribute(String),
}

impl ExtractionType {
    fn extract<N: HtmlNode>(&self, source: &N) -> Result<String, AttributeNotFound> {
        match self {
            ExtractionType::Text => Ok(source.text_contents()),
            ExtractionType::Attribute(name) => source
                .attribute(name)
                .map(|a| a.to_string())
                .ok_or_else(|| AttributeNotFound { name: name.clone() }),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum FromHtmlTextError<A: Error, B: Error> {
    ExtractionFailed(A),
    TextParseError(B),
}

#[cfg(test)]
mod test {

    // TODO test
}
