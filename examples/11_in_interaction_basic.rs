use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let intent = Intent::new()?;
    let response = IntentResponse::new()?;
    let mut interaction = Interaction::new(&intent, Some(&response))?;
    interaction.set_identifier("demo-interaction")?;
    interaction.set_group_identifier("demo-group")?;
    interaction.set_direction(InteractionDirection::Outgoing)?;
    interaction.set_date_interval(1_700_000_000.0, 1_700_000_030.0)?;

    Interaction::delete_by_identifiers(&["demo-interaction"])?;
    Interaction::delete_by_group_identifier("demo-group")?;

    println!("interaction id: {:?}", interaction.identifier());
    println!("interaction group: {:?}", interaction.group_identifier());
    println!("✅ INInteraction OK");
    Ok(())
}
