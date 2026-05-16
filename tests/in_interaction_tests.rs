use intents::prelude::*;

#[test]
fn interaction_properties_and_cleanup_paths_work() -> Result<(), Box<dyn std::error::Error>> {
    let intent = Intent::new()?;
    let response = IntentResponse::new()?;
    let mut interaction = Interaction::new(&intent, Some(&response))?;
    interaction.set_identifier("demo-interaction")?;
    interaction.set_group_identifier("demo-group")?;
    interaction.set_direction(InteractionDirection::Incoming)?;
    interaction.set_date_interval(1_700_000_000.0, 1_700_000_030.0)?;

    assert_eq!(interaction.identifier().as_deref(), Some("demo-interaction"));
    assert_eq!(interaction.group_identifier().as_deref(), Some("demo-group"));
    assert!(matches!(interaction.direction(), InteractionDirection::Incoming));
    assert!(interaction.date_interval().is_some());

    Interaction::delete_by_identifiers(&["demo-interaction"])?;
    Interaction::delete_by_group_identifier("demo-group")?;
    Ok(())
}
