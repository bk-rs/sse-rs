use core::time::Duration;
use std::string;

use async_interval::{intervalable_iter_stream, Intervalable};
use futures_util::{Stream, StreamExt as _};

//
pub fn keep_alive_stream<EVENT, S, INTVL>(
    inner: S,
    interval: Duration,
) -> impl Stream<Item = String>
where
    EVENT: string::ToString,
    S: Stream<Item = EVENT> + Send + 'static,
    INTVL: Intervalable + Send + 'static,
{
    let option = KeepAliveOption::new().interval(interval);

    keep_alive_stream_with_option::<_, _, INTVL>(inner, option)
}

pub fn keep_alive_stream_with_option<EVENT, S, INTVL>(
    inner: S,
    option: KeepAliveOption,
) -> impl Stream<Item = String>
where
    EVENT: string::ToString,
    S: Stream<Item = EVENT> + Send + 'static,
    INTVL: Intervalable + Send + 'static,
{
    let st1 = inner.map(|event| event.to_string());

    let interval = option.get_interval();
    let comment_prefix = option.get_comment_prefix();

    let st2 = intervalable_iter_stream(0..usize::MAX, INTVL::interval(interval))
        .map(move |i| format!(": {}{}\n\n", comment_prefix, i));

    futures_stream_select_ext::select_until_left_is_done(st1, st2)
}

//
#[derive(Debug, Clone, Default)]
pub struct KeepAliveOption {
    interval: Option<Duration>,
    comment_prefix: Option<String>,
}

impl KeepAliveOption {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn interval(mut self, dur: Duration) -> Self {
        self.interval = Some(dur);
        self
    }

    pub fn comment_prefix(mut self, s: String) -> Self {
        self.comment_prefix = Some(s);
        self
    }

    pub fn get_interval(&self) -> Duration {
        self.interval.unwrap_or_else(|| Duration::from_secs(30))
    }

    pub fn get_comment_prefix(&self) -> String {
        self.comment_prefix.to_owned().unwrap_or_else(|| "".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_keep_alive_stream_with_tokio_interval() {
        use futures_util::{stream, StreamExt as _};

        //
        let st = keep_alive_stream::<_, _, tokio::time::Interval>(
            stream::iter(vec!["a", "b"])
                .then(move |x| async move {
                    tokio::time::sleep(tokio::time::Duration::from_micros(2)).await;
                    x
                })
                .map(|x| format!(": {}\n\n", x)),
            Duration::from_micros(1),
        );

        let ret = st.collect::<Vec<_>>().await;
        assert!(ret.contains(&": a\n\n".to_string()));
        assert!(ret.contains(&": b\n\n".to_string()));
        assert!(ret.contains(&": 0\n\n".to_string()));

        //
        let st = keep_alive_stream_with_option::<_, _, tokio::time::Interval>(
            stream::iter(vec!["a", "b"])
                .then(move |x| async move {
                    tokio::time::sleep(tokio::time::Duration::from_micros(2)).await;
                    x
                })
                .map(|x| format!(": {}\n\n", x)),
            KeepAliveOption::new()
                .interval(Duration::from_micros(1))
                .comment_prefix("Ping ".into()),
        );

        let ret = st.collect::<Vec<_>>().await;
        assert!(ret.contains(&": a\n\n".to_string()));
        assert!(ret.contains(&": b\n\n".to_string()));
        assert!(ret.contains(&": Ping 0\n\n".to_string()));
    }
}
