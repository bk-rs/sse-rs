use core::fmt;
use std::borrow::Cow;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Event<'a> {
    //
    pub annotation: Option<Cow<'a, str>>,
    //
    pub retry: Option<usize>,
    //
    pub id: Option<Cow<'a, str>>,
    pub r#type: Option<Cow<'a, str>>,
    pub data: Option<EventData<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventData<'a> {
    Line(Cow<'a, str>),
    Lines(Vec<Cow<'a, str>>),
}

impl<'a> Event<'a> {
    pub fn new(
        data: &'a str,
        r#type: impl Into<Option<&'a str>>,
        id: impl Into<Option<&'a str>>,
    ) -> Self {
        Self {
            id: id.into().map(Cow::Borrowed),
            r#type: r#type.into().map(Cow::Borrowed),
            data: Some(EventData::Line(Cow::Borrowed(data))),
            ..Default::default()
        }
    }

    pub fn with_multiline_data(
        data: Vec<&'a str>,
        r#type: impl Into<Option<&'a str>>,
        id: impl Into<Option<&'a str>>,
    ) -> Self {
        Self {
            id: id.into().map(Cow::Borrowed),
            r#type: r#type.into().map(Cow::Borrowed),
            data: Some(EventData::Lines(
                data.into_iter().map(Cow::Borrowed).collect(),
            )),
            ..Default::default()
        }
    }

    pub fn with_annotation(annotation: &'a str) -> Self {
        Self {
            annotation: Some(Cow::Borrowed(annotation)),
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

impl<'a> fmt::Display for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(annotation) = &self.annotation {
            writeln!(f, ": {}", annotation)?;
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

    #[test]
    fn test_to_string() {
        assert_eq!(
            Event::with_annotation("This is a comment")
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
