use intents::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let airline = Airline::new(Some("Test Air"), Some("TA"), Some("TST"))?;
    let airport = Airport::new(Some("Test Airport"), Some("TAP"), Some("TSTA"))?;
    let gate = AirportGate::new(&airport, Some("A"), Some("1"))?;
    let duration = DateComponentsRange::new_empty()?;
    let flight = Flight::new(&airline, "TA1", &duration, &gate, &gate)?;
    let payment_method = PaymentMethod::apple_pay()?;
    let mut activity = UserActivity::new("com.doomfish.intents-rs.reservation")?;
    activity.set_suggested_invocation_phrase("Check in")?;
    let action = ReservationAction::new(ReservationActionType::CheckIn, &duration, &activity)?;
    let seat = Seat::new(Some("Main"), Some("1"), Some("A"), Some("Window"))?;
    let event = TicketedEvent::new(TicketedEventCategory::Movie, "Movie", &duration, None)?;

    println!("airline: {:?}", airline.name());
    println!("flight: {:?}", flight.flight_number());
    println!("payment: {:?}", payment_method.payment_method_type());
    println!("action: {:?}", action.action_type());
    println!("seat: {:?}", seat.seat_number());
    println!("event: {:?} {:?}", event.category(), event.name());
    println!("✅ reservation OK");
    Ok(())
}
