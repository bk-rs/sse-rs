use alloc::{boxed::Box, vec::Vec};
use core::fmt;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Event {
    //
    pub comment: Option<Box<str>>,
    //
    pub retry: Option<usize>,
    //
    pub id: Option<Box<str>>,
    pub r#type: Option<Box<str>>,
    pub data: Option<EventData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventData {
    Line(Box<str>),
    Lines(Vec<Box<str>>),
}

impl Event {
    pub fn new<'a>(
        data: &str,
        r#type: impl Into<Option<&'a str>>,
        id: impl Into<Option<&'a str>>,
    ) -> Self {
        Self {
            id: id.into().map(Into::into),
            r#type: r#type.into().map(Into::into),
            data: Some(EventData::Line(data.into())),
            ..Default::default()
        }
    }

    pub fn with_multiline_data<'a>(
        data: Vec<&str>,
        r#type: impl Into<Option<&'a str>>,
        id: impl Into<Option<&'a str>>,
    ) -> Self {
        Self {
            id: id.into().map(Into::into),
            r#type: r#type.into().map(Into::into),
            data: Some(EventData::Lines(data.into_iter().map(Into::into).collect())),
            ..Default::default()
        }
    }

    pub fn with_comment(comment: &str) -> Self {
        Self {
            comment: Some(comment.into()),
            ..Default::default()
        }
    }

    pub fn with_retry(retry_ms: usize) -> Self {
        Self {
            retry: Some(retry_ms),
            ..Default::default()
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(comment) = &self.comment {
            writeln!(f, ": {}", comment)?;
        }

        if let Some(retry) = &self.retry {
            writeln!(f, "retry: {}", retry)?;
        }

        if let Some(id) = &self.id {
            writeln!(f, "id: {}", id)?;
        }

        if let Some(r#type) = &self.r#type {
            writeln!(f, "event: {}", r#type)?;
        }

        if let Some(data) = &self.data {
            match &data {
                EventData::Line(s) => {
                    for x in s.lines() {
                        writeln!(f, "data: {}", x)?;
                    }
                }
                &EventData::Lines(s_vec) => {
                    for s in s_vec {
                        for x in s.lines() {
                            writeln!(f, "data: {}", x)?;
                        }
                    }
                }
            }
        }

        writeln!(f)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::{string::ToString as _, vec};

    #[test]
    fn test_to_string() {
        assert_eq!(
            Event::with_comment("This is a comment")
                .to_string()
                .as_bytes(),
            b": This is a comment\n\n"
        );

        assert_eq!(
            Event::with_retry(10000).to_string().as_bytes(),
            b"retry: 10000\n\n"
        );

        assert_eq!(
            Event::new("message", None, None).to_string().as_bytes(),
            b"data: message\n\n"
        );
        assert_eq!(
            Event::new(r#"Message of type "foo""#, "foo", None)
                .to_string()
                .as_bytes(),
            b"event: foo\ndata: Message of type \"foo\"\n\n"
        );
        assert_eq!(
            Event::new(r#"Message of type "foo""#, "foo", "1")
                .to_string()
                .as_bytes(),
            b"id: 1\nevent: foo\ndata: Message of type \"foo\"\n\n"
        );

        assert_eq!(
            Event::with_multiline_data(
                vec!["Multi-line message of", r#"type "bar" and id "42""#],
                "bar",
                "42"
            )
            .to_string()
            .as_bytes(),
            b"id: 42\nevent: bar\ndata: Multi-line message of\ndata: type \"bar\" and id \"42\"\n\n"
        );

        assert_eq!(
            Event::with_multiline_data(vec!["another message", "with two lines "], None, None)
                .to_string()
                .as_bytes(),
            b"data: another message\ndata: with two lines \n\n"
        );
    }
}
