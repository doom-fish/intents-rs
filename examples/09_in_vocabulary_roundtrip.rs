use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vocabulary = IntentVocabulary::shared()?;
    let speakable = SpeakableString::new("demo-contact", "Demo Contact", None)?;

    vocabulary.set_vocabulary_strings(&["Demo Group"], VocabularyStringType::ContactGroupName)?;
    vocabulary.set_vocabulary_speakables(&[&speakable], VocabularyStringType::ContactName)?;
    vocabulary.remove_all_vocabulary_strings()?;

    println!("vocabulary class: {}", vocabulary.class_name());
    println!("✅ INVocabulary OK");
    Ok(())
}
