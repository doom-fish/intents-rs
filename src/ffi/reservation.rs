use core::ffi::{c_char, c_void};

extern "C" {
    pub fn inx_airline_create(
        name: *const c_char,
        iata_code: *const c_char,
        icao_code: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_airport_create(
        name: *const c_char,
        iata_code: *const c_char,
        icao_code: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_airport_gate_create(
        airport: *mut c_void,
        terminal: *const c_char,
        gate: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_currency_amount_create(
        amount: f64,
        currency_code: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_date_components_range_create_empty(out_error: *mut *mut c_char) -> *mut c_void;
    pub fn inx_flight_create(
        airline: *mut c_void,
        flight_number: *const c_char,
        flight_duration: *mut c_void,
        departure_airport_gate: *mut c_void,
        arrival_airport_gate: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_payment_method_create(
        payment_method_type: i64,
        name: *const c_char,
        identification_hint: *const c_char,
        icon: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_payment_method_copy_apple_pay() -> *mut c_void;
    pub fn inx_rental_car_create(
        rental_company_name: *const c_char,
        rental_car_type: *const c_char,
        make: *const c_char,
        model: *const c_char,
        rental_car_description: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_reservation_action_create(
        action_type: i64,
        valid_duration: *mut c_void,
        user_activity: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_seat_create(
        seat_section: *const c_char,
        seat_row: *const c_char,
        seat_number: *const c_char,
        seating_type: *const c_char,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn inx_ticketed_event_create(
        category: i64,
        name: *const c_char,
        event_duration: *mut c_void,
        location: *mut c_void,
        out_error: *mut *mut c_char,
    ) -> *mut c_void;
}
