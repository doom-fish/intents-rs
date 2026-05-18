use core::ffi::c_void;

use crate::error::IntentsError;
use crate::ffi;
use crate::in_object::Image;
use crate::intent_response::UserActivity;
use crate::private::{self, RawObject, RetainedObject};
use crate::support::Placemark;

macro_rules! simple_enum {
    ($name:ident { $($variant:ident = $raw:expr,)* }) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum $name {
            $($variant,)*
            Other(i64),
        }

        impl $name {
            #[allow(dead_code)]
            pub(crate) const fn from_raw(raw: i64) -> Self {
                match raw {
                    $($raw => Self::$variant,)*
                    other => Self::Other(other),
                }
            }

            #[allow(dead_code)]
            pub(crate) const fn raw_value(self) -> i64 {
                match self {
                    $(Self::$variant => $raw,)*
                    Self::Other(raw) => raw,
                }
            }
        }
    };
}

macro_rules! objc_wrapper {
    ($name:ident, $objc_class:literal, $context:literal) => {
        #[derive(Debug)]
        pub struct $name {
            raw: RetainedObject,
        }

        impl $name {
            pub const OBJC_CLASS: &'static str = $objc_class;

            #[allow(dead_code)]
            pub(crate) unsafe fn from_owned(ptr: *mut c_void) -> Result<Self, IntentsError> {
                Ok(Self {
                    raw: unsafe { RetainedObject::from_owned(ptr, $context) }?,
                })
            }

            #[allow(dead_code)]
            pub(crate) const fn from_retained(raw: RetainedObject) -> Self {
                Self { raw }
            }

            #[allow(dead_code)]
            pub(crate) fn new_blank() -> Result<Self, IntentsError> {
                Ok(Self::from_retained(private::create_blank_object(
                    Self::OBJC_CLASS,
                    $context,
                )?))
            }

            pub fn class_name(&self) -> String {
                private::class_name(self)
            }
        }

        impl RawObject for $name {
            fn as_ptr(&self) -> *mut c_void {
                self.raw.as_ptr()
            }
        }
    };
}

simple_enum!(PaymentMethodType {
    Unknown = 0,
    Checking = 1,
    Savings = 2,
    Brokerage = 3,
    Debit = 4,
    Credit = 5,
    Prepaid = 6,
    Store = 7,
    ApplePay = 8,
});

simple_enum!(RecurrenceFrequency {
    Unknown = 0,
    Minute = 1,
    Hourly = 2,
    Daily = 3,
    Weekly = 4,
    Monthly = 5,
    Yearly = 6,
});

simple_enum!(ReservationActionType {
    Unknown = 0,
    CheckIn = 1,
});

simple_enum!(ReservationStatus {
    Unknown = 0,
    Canceled = 1,
    Pending = 2,
    Hold = 3,
    Confirmed = 4,
});

simple_enum!(TicketedEventCategory {
    Unknown = 0,
    Movie = 1,
});

objc_wrapper!(Airline, "INAirline", "airline");
objc_wrapper!(Airport, "INAirport", "airport");
objc_wrapper!(AirportGate, "INAirportGate", "airport gate");
objc_wrapper!(BoatReservation, "INBoatReservation", "boat reservation");
objc_wrapper!(BoatTrip, "INBoatTrip", "boat trip");
objc_wrapper!(BusReservation, "INBusReservation", "bus reservation");
objc_wrapper!(BusTrip, "INBusTrip", "bus trip");
objc_wrapper!(CurrencyAmount, "INCurrencyAmount", "currency amount");
objc_wrapper!(
    DateComponentsRange,
    "INDateComponentsRange",
    "date components range"
);
objc_wrapper!(Flight, "INFlight", "flight");
objc_wrapper!(
    FlightReservation,
    "INFlightReservation",
    "flight reservation"
);
objc_wrapper!(
    LodgingReservation,
    "INLodgingReservation",
    "lodging reservation"
);
objc_wrapper!(PaymentMethod, "INPaymentMethod", "payment method");
objc_wrapper!(RentalCar, "INRentalCar", "rental car");
objc_wrapper!(
    RentalCarReservation,
    "INRentalCarReservation",
    "rental car reservation"
);
objc_wrapper!(Reservation, "INReservation", "reservation");
objc_wrapper!(
    ReservationAction,
    "INReservationAction",
    "reservation action"
);
objc_wrapper!(
    RestaurantReservation,
    "INRestaurantReservation",
    "restaurant reservation"
);
objc_wrapper!(Seat, "INSeat", "seat");
objc_wrapper!(TicketedEvent, "INTicketedEvent", "ticketed event");
objc_wrapper!(
    TicketedEventReservation,
    "INTicketedEventReservation",
    "ticketed event reservation"
);
objc_wrapper!(TrainReservation, "INTrainReservation", "train reservation");
objc_wrapper!(TrainTrip, "INTrainTrip", "train trip");

impl Airline {
    pub fn new(
        name: Option<&str>,
        iata_code: Option<&str>,
        icao_code: Option<&str>,
    ) -> Result<Self, IntentsError> {
        let name = name
            .map(|value| private::cstring(value, "airline name"))
            .transpose()?;
        let iata_code = iata_code
            .map(|value| private::cstring(value, "airline iata code"))
            .transpose()?;
        let icao_code = icao_code
            .map(|value| private::cstring(value, "airline icao code"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_airline_create(
                name.as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                iata_code
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                icao_code
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating airline") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn name(&self) -> Option<String> {
        private::string_property(self, "name")
    }

    pub fn iata_code(&self) -> Option<String> {
        private::string_property(self, "iataCode")
    }

    pub fn icao_code(&self) -> Option<String> {
        private::string_property(self, "icaoCode")
    }
}

impl Airport {
    pub fn new(
        name: Option<&str>,
        iata_code: Option<&str>,
        icao_code: Option<&str>,
    ) -> Result<Self, IntentsError> {
        let name = name
            .map(|value| private::cstring(value, "airport name"))
            .transpose()?;
        let iata_code = iata_code
            .map(|value| private::cstring(value, "airport iata code"))
            .transpose()?;
        let icao_code = icao_code
            .map(|value| private::cstring(value, "airport icao code"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_airport_create(
                name.as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                iata_code
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                icao_code
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating airport") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn name(&self) -> Option<String> {
        private::string_property(self, "name")
    }
}

impl AirportGate {
    pub fn new(
        airport: &Airport,
        terminal: Option<&str>,
        gate: Option<&str>,
    ) -> Result<Self, IntentsError> {
        let terminal = terminal
            .map(|value| private::cstring(value, "airport gate terminal"))
            .transpose()?;
        let gate = gate
            .map(|value| private::cstring(value, "airport gate value"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_airport_gate_create(
                airport.as_ptr(),
                terminal
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                gate.as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating airport gate") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn airport(&self) -> Option<Airport> {
        private::object_property(self, "airport").map(Airport::from_retained)
    }

    pub fn terminal(&self) -> Option<String> {
        private::string_property(self, "terminal")
    }

    pub fn gate(&self) -> Option<String> {
        private::string_property(self, "gate")
    }
}

impl BoatTrip {
    pub fn provider(&self) -> Option<String> {
        private::string_property(self, "provider")
    }
}

impl BusTrip {
    pub fn provider(&self) -> Option<String> {
        private::string_property(self, "provider")
    }
}

impl CurrencyAmount {
    pub fn new(amount: f64, currency_code: &str) -> Result<Self, IntentsError> {
        let currency_code = private::cstring(currency_code, "currency amount currency code")?;
        let mut error = std::ptr::null_mut();
        let ptr =
            unsafe { ffi::inx_currency_amount_create(amount, currency_code.as_ptr(), &mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating currency amount") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn amount(&self) -> Option<f64> {
        private::double_property(self, "amount")
    }

    pub fn currency_code(&self) -> Option<String> {
        private::string_property(self, "currencyCode")
    }
}

impl DateComponentsRange {
    pub fn new_empty() -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe { ffi::inx_date_components_range_create_empty(&mut error) };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating date components range") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }
}

impl Flight {
    pub fn new(
        airline: &Airline,
        flight_number: &str,
        flight_duration: &DateComponentsRange,
        departure_airport_gate: &AirportGate,
        arrival_airport_gate: &AirportGate,
    ) -> Result<Self, IntentsError> {
        let flight_number = private::cstring(flight_number, "flight number")?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_flight_create(
                airline.as_ptr(),
                flight_number.as_ptr(),
                flight_duration.as_ptr(),
                departure_airport_gate.as_ptr(),
                arrival_airport_gate.as_ptr(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating flight") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn flight_number(&self) -> Option<String> {
        private::string_property(self, "flightNumber")
    }

    pub fn airline(&self) -> Option<Airline> {
        private::object_property(self, "airline").map(Airline::from_retained)
    }
}

impl PaymentMethod {
    pub fn new(
        payment_method_type: PaymentMethodType,
        name: Option<&str>,
        identification_hint: Option<&str>,
        icon: Option<&Image>,
    ) -> Result<Self, IntentsError> {
        let name = name
            .map(|value| private::cstring(value, "payment method name"))
            .transpose()?;
        let identification_hint = identification_hint
            .map(|value| private::cstring(value, "payment method identification hint"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_payment_method_create(
                payment_method_type.raw_value(),
                name.as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                identification_hint
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                icon.map_or(std::ptr::null_mut(), RawObject::as_ptr),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating payment method") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn apple_pay() -> Result<Self, IntentsError> {
        let ptr = unsafe { ffi::inx_payment_method_copy_apple_pay() };
        unsafe { Self::from_owned(ptr) }
    }

    pub fn payment_method_type(&self) -> PaymentMethodType {
        private::integer_property(self, "type")
            .map_or(PaymentMethodType::Unknown, PaymentMethodType::from_raw)
    }

    pub fn name(&self) -> Option<String> {
        private::string_property(self, "name")
    }

    pub fn identification_hint(&self) -> Option<String> {
        private::string_property(self, "identificationHint")
    }

    pub fn icon(&self) -> Option<Image> {
        private::object_property(self, "icon").map(Image::from_retained)
    }
}

impl RentalCar {
    pub fn new(
        rental_company_name: &str,
        rental_car_type: Option<&str>,
        make: Option<&str>,
        model: Option<&str>,
        rental_car_description: Option<&str>,
    ) -> Result<Self, IntentsError> {
        let rental_company_name = private::cstring(rental_company_name, "rental car company name")?;
        let rental_car_type = rental_car_type
            .map(|value| private::cstring(value, "rental car type"))
            .transpose()?;
        let make = make
            .map(|value| private::cstring(value, "rental car make"))
            .transpose()?;
        let model = model
            .map(|value| private::cstring(value, "rental car model"))
            .transpose()?;
        let rental_car_description = rental_car_description
            .map(|value| private::cstring(value, "rental car description"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_rental_car_create(
                rental_company_name.as_ptr(),
                rental_car_type
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                make.as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                model
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                rental_car_description
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating rental car") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn rental_company_name(&self) -> Option<String> {
        private::string_property(self, "rentalCompanyName")
    }
}

impl Reservation {
    pub fn reservation_number(&self) -> Option<String> {
        private::string_property(self, "reservationNumber")
    }

    pub fn reservation_status(&self) -> ReservationStatus {
        private::integer_property(self, "reservationStatus")
            .map_or(ReservationStatus::Unknown, ReservationStatus::from_raw)
    }

    pub fn reservation_holder_name(&self) -> Option<String> {
        private::string_property(self, "reservationHolderName")
    }

    pub fn actions_count(&self) -> usize {
        private::array_count_property(self, "actions").unwrap_or_default()
    }

    pub fn url(&self) -> Option<String> {
        private::string_property(self, "URL")
    }
}

impl ReservationAction {
    pub fn new(
        action_type: ReservationActionType,
        valid_duration: &DateComponentsRange,
        user_activity: &UserActivity,
    ) -> Result<Self, IntentsError> {
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_reservation_action_create(
                action_type.raw_value(),
                valid_duration.as_ptr(),
                user_activity.as_ptr(),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating reservation action") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn action_type(&self) -> ReservationActionType {
        private::integer_property(self, "type").map_or(
            ReservationActionType::Unknown,
            ReservationActionType::from_raw,
        )
    }

    pub fn valid_duration(&self) -> Option<DateComponentsRange> {
        private::object_property(self, "validDuration").map(DateComponentsRange::from_retained)
    }

    pub fn user_activity(&self) -> Option<UserActivity> {
        private::object_property(self, "userActivity").map(UserActivity::from_retained)
    }
}

impl Seat {
    pub fn new(
        seat_section: Option<&str>,
        seat_row: Option<&str>,
        seat_number: Option<&str>,
        seating_type: Option<&str>,
    ) -> Result<Self, IntentsError> {
        let seat_section = seat_section
            .map(|value| private::cstring(value, "seat section"))
            .transpose()?;
        let seat_row = seat_row
            .map(|value| private::cstring(value, "seat row"))
            .transpose()?;
        let seat_number = seat_number
            .map(|value| private::cstring(value, "seat number"))
            .transpose()?;
        let seating_type = seating_type
            .map(|value| private::cstring(value, "seat seating type"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_seat_create(
                seat_section
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                seat_row
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                seat_number
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                seating_type
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating seat") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn seat_number(&self) -> Option<String> {
        private::string_property(self, "seatNumber")
    }
}

impl TicketedEvent {
    pub fn new(
        category: TicketedEventCategory,
        name: &str,
        event_duration: &DateComponentsRange,
        location: Option<&Placemark>,
    ) -> Result<Self, IntentsError> {
        let name = private::cstring(name, "ticketed event name")?;
        let mut error = std::ptr::null_mut();
        let ptr = unsafe {
            ffi::inx_ticketed_event_create(
                category.raw_value(),
                name.as_ptr(),
                event_duration.as_ptr(),
                location.map_or(std::ptr::null_mut(), RawObject::as_ptr),
                &mut error,
            )
        };
        if ptr.is_null() {
            Err(unsafe { private::take_error(error, "creating ticketed event") })
        } else {
            unsafe { Self::from_owned(ptr) }
        }
    }

    pub fn category(&self) -> TicketedEventCategory {
        private::integer_property(self, "category").map_or(
            TicketedEventCategory::Unknown,
            TicketedEventCategory::from_raw,
        )
    }

    pub fn name(&self) -> Option<String> {
        private::string_property(self, "name")
    }
}

impl TrainTrip {
    pub fn provider(&self) -> Option<String> {
        private::string_property(self, "provider")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reservation_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let airline = Airline::new(Some("Test Air"), Some("TA"), Some("TST"))?;
        let airport = Airport::new(Some("Test Airport"), Some("TAP"), Some("TSTA"))?;
        let gate = AirportGate::new(&airport, Some("A"), Some("1"))?;
        let duration = DateComponentsRange::new_empty()?;
        let flight = Flight::new(&airline, "TA1", &duration, &gate, &gate)?;
        let payment_method = PaymentMethod::apple_pay()?;
        let seat = Seat::new(Some("Main"), Some("1"), Some("A"), Some("Window"))?;
        let event = TicketedEvent::new(TicketedEventCategory::Movie, "Movie", &duration, None)?;

        assert_eq!(airline.name().as_deref(), Some("Test Air"));
        assert_eq!(flight.flight_number().as_deref(), Some("TA1"));
        assert_eq!(
            payment_method.payment_method_type(),
            PaymentMethodType::ApplePay
        );
        assert_eq!(seat.seat_number().as_deref(), Some("A"));
        assert_eq!(event.category(), TicketedEventCategory::Movie);
        Ok(())
    }
}
