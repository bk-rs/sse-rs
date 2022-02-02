pub mod sleep;

//
#[cfg(any(feature = "stream_sleep_tokio", feature = "stream_sleep_async_timer"))]
pub fn keep_alive_stream<EVENT, S>(
    stream: S,
    interval: core::time::Duration,
) -> impl futures_util::Stream<Item = String>
where
    EVENT: std::string::ToString,
    S: futures_util::Stream<Item = EVENT> + Send + 'static,
{
    use futures_util::{stream, StreamExt as _};

    let st1 = stream.map(|event| event.to_string());

    let st2 = stream::iter(0..usize::MAX)
        .then(move |i| async move {
            self::sleep::sleep(interval).await;
            i
        })
        .map(|i| format!(": {}\n\n", i));

    futures_stream_select_ext::select_until_left_is_done(st1, st2).boxed()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[cfg(all(
        feature = "stream_sleep_tokio",
        not(feature = "stream_sleep_async_timer")
    ))]
    #[tokio::test]
    async fn test_keep_alive_stream_with_sleep_tokio() {
        use core::time::Duration;

        use futures_util::{stream, StreamExt as _};

        let st = keep_alive_stream(
            stream::iter(vec!["a", "b"])
                .then(move |x| async move {
                    tokio::time::sleep(tokio::time::Duration::from_micros(2)).await;
                    x
                })
                .map(|x| format!(": {}\n\n", x)),
            Duration::from_micros(1),
        );

        let ret = st.collect::<Vec<_>>().await;

        assert!(ret.contains(&": 0\n\n".to_string()));
    }
}
