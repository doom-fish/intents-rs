//! Async Siri authorization request.
//!
//! Demonstrates `AsyncPreferences::request_siri_authorization` using
//! `pollster::block_on`. On a headless system or when `INPreferences` is
//! unavailable this exits gracefully.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        use intents::async_api::AsyncPreferences;
        use intents::{Preferences, SiriAuthorizationStatus};

        // Avoid showing a system dialog in headless / CI environments.
        if matches!(
            Preferences::siri_authorization_status(),
            SiriAuthorizationStatus::NotDetermined
        ) {
            println!("Siri authorization is NotDetermined — skipping dialog-triggering call");
            return Ok(());
        }

        match AsyncPreferences::request_siri_authorization().await {
            Ok(status) => println!("Siri authorization status: {status:?}"),
            Err(e) => {
                println!(
                    "request_siri_authorization: {e} \
                     (non-fatal — INPreferences may be unavailable)"
                );
            }
        }

        Ok(())
    })
}
