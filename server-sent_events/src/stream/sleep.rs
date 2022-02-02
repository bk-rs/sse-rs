cfg_if::cfg_if! {
    if #[cfg(feature = "stream_sleep_tokio")] {
        pub use self::with_tokio::sleep;
    } else if #[cfg(feature = "stream_sleep_async_timer")] {
        pub use self::with_async_lock::sleep;
    } else {
        pub use self::with_std::sleep;
    }
}

#[cfg(feature = "stream_sleep_tokio")]
pub mod with_tokio {
    pub async fn sleep(dur: core::time::Duration) {
        tokio::time::sleep(tokio::time::Duration::from_secs(dur.as_secs())).await
    }
}

#[cfg(feature = "stream_sleep_async_timer")]
pub mod with_async_lock {
    pub async fn sleep(dur: core::time::Duration) {
        async_timer::interval(dur).wait().await
    }
}

pub mod with_std {
    pub async fn sleep(dur: core::time::Duration) {
        std::thread::sleep(dur)
    }
}
