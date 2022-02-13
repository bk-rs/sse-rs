pub fn content_type_value_is_sse(value: &str) -> bool {
    #[cfg(all(feature = "std", feature = "mime"))]
    {
        if let Ok(m) = value.parse::<mime::Mime>() {
            if m.type_() == mime::TEXT && m.subtype() == mime::EVENT_STREAM {
                return true;
            }
        }

        false
    }
    #[cfg(not(all(feature = "std", feature = "mime")))]
    {
        value == crate::CONTENT_TYPE_VALUE
    }
}

#[cfg(all(feature = "std", feature = "http"))]
pub fn content_type_is_sse(headers: &http::HeaderMap<http::HeaderValue>) -> bool {
    if let Some(header_value) = headers.get(http::header::CONTENT_TYPE) {
        if let Ok(value) = core::str::from_utf8(header_value.as_bytes()) {
            return content_type_value_is_sse(value);
        }
    }

    false
}

#[cfg(all(feature = "std", feature = "http"))]
pub fn get_last_event_id(
    headers: &http::HeaderMap<http::HeaderValue>,
) -> Result<Option<String>, core::str::Utf8Error> {
    if let Some(header_value) = headers.get(crate::LAST_EVENT_ID_HEADER_KEY) {
        if let Ok(value) = core::str::from_utf8(header_value.as_bytes()) {
            return Ok(Some(value.into()));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_type_value_is_sse() {
        assert!(content_type_value_is_sse("text/event-stream"));
        assert!(!content_type_value_is_sse("text/plain"));
    }

    #[cfg(all(feature = "std", feature = "http"))]
    #[test]
    fn test_content_type_is_sse() {
        assert!(content_type_is_sse(
            http::Response::builder()
                .header("Content-Type", "text/event-stream")
                .body(())
                .unwrap()
                .headers()
        ));
        assert!(!content_type_is_sse(
            http::Response::builder()
                .header("Content-Type", "text/plain")
                .body(())
                .unwrap()
                .headers()
        ));
        assert!(!content_type_is_sse(
            http::Response::builder().body(()).unwrap().headers()
        ));
    }

    #[cfg(all(feature = "std", feature = "http"))]
    #[test]
    fn test_get_last_event_id() {
        assert_eq!(
            get_last_event_id(
                http::Request::builder()
                    .header("Last-Event-ID", "foo")
                    .body(())
                    .unwrap()
                    .headers()
            ),
            Ok(Some("foo".into()))
        );
        assert_eq!(
            get_last_event_id(http::Request::builder().body(()).unwrap().headers()),
            Ok(None)
        );
    }
}
