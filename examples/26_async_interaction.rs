//! Async interaction donation and deletion.
//!
//! Demonstrates `AsyncInteraction::donate`, `::delete`, `::delete_by_group`,
//! and `::delete_all` using `pollster::block_on`.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        use intents::async_api::AsyncInteraction;
        use intents::{Intent, Interaction};

        let intent = Intent::new()?;
        let interaction = Interaction::new(&intent, None)?;

        // donate — may fail on headless CI (no Siri daemon), treat as advisory
        match AsyncInteraction::donate(&interaction).await {
            Ok(()) => println!("donate: ok"),
            Err(e) => println!("donate: {e} (non-fatal on headless system)"),
        }

        // delete by identifier (may not exist; that is fine)
        match AsyncInteraction::delete(&["async-example-id"])?.await {
            Ok(()) => println!("delete by id: ok"),
            Err(e) => println!("delete by id: {e} (non-fatal)"),
        }

        // delete by group
        match AsyncInteraction::delete_by_group("async-example-group")?.await {
            Ok(()) => println!("delete by group: ok"),
            Err(e) => println!("delete by group: {e} (non-fatal)"),
        }

        // delete all
        match AsyncInteraction::delete_all().await {
            Ok(()) => println!("delete_all: ok"),
            Err(e) => println!("delete_all: {e} (non-fatal)"),
        }

        Ok(())
    })
}
