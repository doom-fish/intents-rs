import CoreLocation
import Foundation
import Intents

@_cdecl("inx_airline_create")
public func inx_airline_create(
    _ name: UnsafePointer<CChar>?,
    _ iataCode: UnsafePointer<CChar>?,
    _ icaoCode: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    inxRetain(INAirline(
        name: name.map { String(cString: $0) },
        iataCode: iataCode.map { String(cString: $0) },
        icaoCode: icaoCode.map { String(cString: $0) }
    ))
}

@_cdecl("inx_airport_create")
public func inx_airport_create(
    _ name: UnsafePointer<CChar>?,
    _ iataCode: UnsafePointer<CChar>?,
    _ icaoCode: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    inxRetain(INAirport(
        name: name.map { String(cString: $0) },
        iataCode: iataCode.map { String(cString: $0) },
        icaoCode: icaoCode.map { String(cString: $0) }
    ))
}

@_cdecl("inx_airport_gate_create")
public func inx_airport_gate_create(
    _ airportPtr: UnsafeMutableRawPointer?,
    _ terminal: UnsafePointer<CChar>?,
    _ gate: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let airportPtr else {
        outError?.pointee = inxCString("airport gate airport pointer was NULL")
        return nil
    }

    let airport: INAirport = inxUnretained(airportPtr)
    let airportGate = INAirportGate(
        airport: airport,
        terminal: terminal.map { String(cString: $0) },
        gate: gate.map { String(cString: $0) }
    )
    return inxRetain(airportGate)
}

@_cdecl("inx_currency_amount_create")
public func inx_currency_amount_create(
    _ amount: Double,
    _ currencyCode: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard #available(macOS 12.0, *) else {
        outError?.pointee = inxCString("INCurrencyAmount requires macOS 12+")
        return nil
    }
    guard let currencyCode else {
        outError?.pointee = inxCString("currency amount currency code was NULL")
        return nil
    }

    let currencyAmount = INCurrencyAmount(
        amount: NSDecimalNumber(value: amount),
        currencyCode: String(cString: currencyCode)
    )
    return inxRetain(currencyAmount)
}

@_cdecl("inx_date_components_range_create_empty")
public func inx_date_components_range_create_empty(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    inxRetain(INDateComponentsRange(start: nil, end: nil))
}

@_cdecl("inx_flight_create")
public func inx_flight_create(
    _ airlinePtr: UnsafeMutableRawPointer?,
    _ flightNumber: UnsafePointer<CChar>?,
    _ flightDurationPtr: UnsafeMutableRawPointer?,
    _ departureAirportGatePtr: UnsafeMutableRawPointer?,
    _ arrivalAirportGatePtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let airlinePtr, let flightNumber, let flightDurationPtr, let departureAirportGatePtr, let arrivalAirportGatePtr else {
        outError?.pointee = inxCString("flight arguments were incomplete")
        return nil
    }

    let airline: INAirline = inxUnretained(airlinePtr)
    let flightDuration: INDateComponentsRange = inxUnretained(flightDurationPtr)
    let departureAirportGate: INAirportGate = inxUnretained(departureAirportGatePtr)
    let arrivalAirportGate: INAirportGate = inxUnretained(arrivalAirportGatePtr)
    return inxRetain(INFlight(
        airline: airline,
        flightNumber: String(cString: flightNumber),
        boardingTime: nil,
        flightDuration: flightDuration,
        departureAirportGate: departureAirportGate,
        arrivalAirportGate: arrivalAirportGate
    ))
}

@_cdecl("inx_payment_method_create")
public func inx_payment_method_create(
    _ paymentMethodType: Int,
    _ name: UnsafePointer<CChar>?,
    _ identificationHint: UnsafePointer<CChar>?,
    _ iconPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let paymentMethodType = INPaymentMethodType(rawValue: paymentMethodType) else {
        outError?.pointee = inxCString("invalid INPaymentMethodType raw value")
        return nil
    }

    let icon: INImage? = iconPtr.map { inxUnretained($0) }
    return inxRetain(INPaymentMethod(
        type: paymentMethodType,
        name: name.map { String(cString: $0) },
        identificationHint: identificationHint.map { String(cString: $0) },
        icon: icon
    ))
}

@_cdecl("inx_payment_method_copy_apple_pay")
public func inx_payment_method_copy_apple_pay() -> UnsafeMutableRawPointer? {
    inxRetain(INPaymentMethod.applePay())
}

@_cdecl("inx_rental_car_create")
public func inx_rental_car_create(
    _ rentalCompanyName: UnsafePointer<CChar>?,
    _ rentalCarType: UnsafePointer<CChar>?,
    _ make: UnsafePointer<CChar>?,
    _ model: UnsafePointer<CChar>?,
    _ rentalCarDescription: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let rentalCompanyName else {
        outError?.pointee = inxCString("rental car company name was NULL")
        return nil
    }

    return inxRetain(INRentalCar(
        rentalCompanyName: String(cString: rentalCompanyName),
        type: rentalCarType.map { String(cString: $0) },
        make: make.map { String(cString: $0) },
        model: model.map { String(cString: $0) },
        rentalCarDescription: rentalCarDescription.map { String(cString: $0) }
    ))
}

@_cdecl("inx_reservation_action_create")
public func inx_reservation_action_create(
    _ actionType: Int,
    _ validDurationPtr: UnsafeMutableRawPointer?,
    _ userActivityPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let validDurationPtr, let userActivityPtr else {
        outError?.pointee = inxCString("reservation action arguments were incomplete")
        return nil
    }
    guard let actionType = INReservationActionType(rawValue: actionType) else {
        outError?.pointee = inxCString("invalid INReservationActionType raw value")
        return nil
    }

    let validDuration: INDateComponentsRange = inxUnretained(validDurationPtr)
    let userActivity: NSUserActivity = inxUnretained(userActivityPtr)
    return inxRetain(INReservationAction(type: actionType, validDuration: validDuration, userActivity: userActivity))
}

@_cdecl("inx_seat_create")
public func inx_seat_create(
    _ seatSection: UnsafePointer<CChar>?,
    _ seatRow: UnsafePointer<CChar>?,
    _ seatNumber: UnsafePointer<CChar>?,
    _ seatingType: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    let seat = INSeat(
        seatSection: seatSection.map { String(cString: $0) },
        seatRow: seatRow.map { String(cString: $0) },
        seatNumber: seatNumber.map { String(cString: $0) },
        seatingType: seatingType.map { String(cString: $0) }
    )
    return inxRetain(seat)
}

@_cdecl("inx_ticketed_event_create")
public func inx_ticketed_event_create(
    _ category: Int,
    _ name: UnsafePointer<CChar>?,
    _ eventDurationPtr: UnsafeMutableRawPointer?,
    _ locationPtr: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let name, let eventDurationPtr else {
        outError?.pointee = inxCString("ticketed event arguments were incomplete")
        return nil
    }
    guard let category = INTicketedEventCategory(rawValue: category) else {
        outError?.pointee = inxCString("invalid INTicketedEventCategory raw value")
        return nil
    }

    let eventDuration: INDateComponentsRange = inxUnretained(eventDurationPtr)
    let location: CLPlacemark? = locationPtr.map { inxUnretained($0) }
    return inxRetain(INTicketedEvent(
        category: category,
        name: String(cString: name),
        eventDuration: eventDuration,
        location: location
    ))
}
