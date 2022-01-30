use core::time::Duration;
use std::string::ToString;

use futures_stream_select_ext::select_until_left_is_done;
use futures_util::{stream, Stream, StreamExt as _};

//
pub fn keep_alive_stream<EVENT, S>(stream: S, interval: Duration) -> impl Stream<Item = String>
where
    EVENT: ToString,
    S: Stream<Item = EVENT> + Send + 'static,
{
    let st1 = stream.map(|event| event.to_string());

    let st2 = stream::iter(0..usize::MAX)
        .then(move |i| async move {
            async_timer::interval(interval).wait().await;
            i
        })
        .map(|i| format!(": {}", i));

    select_until_left_is_done(st1, st2).boxed()
}
