//! Async voice shortcut retrieval.
//!
//! Demonstrates `AsyncVoiceShortcutCenter::get_all` and `::get` using
//! `pollster::block_on`. `INVoiceShortcutCenter` requires macOS 12+; earlier
//! OS versions exit gracefully.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        use intents::async_api::AsyncVoiceShortcutCenter;
        use intents::VoiceShortcutCenter;

        let center = match VoiceShortcutCenter::shared() {
            Ok(c) => c,
            Err(e) => {
                println!(
                    "VoiceShortcutCenter unavailable: {e} \
                     (requires macOS 12+ — skipping)"
                );
                return Ok(());
            }
        };

        // get all shortcuts
        match AsyncVoiceShortcutCenter::get_all(&center).await {
            Ok(shortcuts) => {
                println!("get_all: {} shortcut(s)", shortcuts.len());
                for s in &shortcuts {
                    println!(
                        "  id={:?}  phrase={:?}",
                        s.identifier(),
                        s.invocation_phrase()
                    );
                }
            }
            Err(e) => println!("get_all: {e} (non-fatal)"),
        }

        // look up a known-missing UUID — should resolve to Ok(None)
        let fake_uuid = "00000000-0000-0000-0000-000000000000";
        match AsyncVoiceShortcutCenter::get(&center, fake_uuid)?.await {
            Ok(None) => println!("get {fake_uuid}: not found (expected)"),
            Ok(Some(s)) => println!("get: found {:?}", s.identifier()),
            Err(e) => println!("get: {e} (non-fatal)"),
        }

        Ok(())
    })
}
