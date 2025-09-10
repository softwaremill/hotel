use crate::models::Booking;
use chrono::NaiveDate;

pub fn can_accommodate_booking(
    hotel_room_count: i32,
    existing_bookings: Vec<Booking>,
    new_start: NaiveDate,
    new_end: NaiveDate,
) -> bool {
    // Create a list of all bookings including the new one (with a dummy ID)
    let mut all_bookings = existing_bookings;
    all_bookings.push(Booking {
        id: -1, // dummy ID for the new booking
        hotel_id: 0, // doesn't matter for this algorithm
        room_number: None, // unassigned
        guest_name: "".to_string(), // doesn't matter
        start_time: new_start,
        end_time: new_end,
        status: crate::models::BookingStatus::Confirmed,
    });

    // Sort bookings by start time
    all_bookings.sort_by(|a, b| a.start_time.cmp(&b.start_time));

    // Try to assign rooms using a greedy algorithm
    assign_rooms_greedy(&all_bookings, hotel_room_count).is_some()
}

fn assign_rooms_greedy(bookings: &[Booking], room_count: i32) -> Option<Vec<Option<i32>>> {
    let mut assignments = vec![None; bookings.len()];
    
    // Track which rooms are occupied at any given time
    // Each room tracks when it becomes free
    let mut room_free_times: Vec<Option<NaiveDate>> = vec![None; room_count as usize];

    for (booking_idx, booking) in bookings.iter().enumerate() {
        let mut assigned = false;
        
        // Try to find an available room
        for room_idx in 0..room_count as usize {
            // Room is available if it's never been used or is free before this booking starts
            if room_free_times[room_idx].is_none() || 
               room_free_times[room_idx].unwrap() <= booking.start_time {
                // Assign this room to the booking
                assignments[booking_idx] = Some(room_idx as i32 + 1); // rooms are 1-indexed
                room_free_times[room_idx] = Some(booking.end_time);
                assigned = true;
                break;
            }
        }
        
        if !assigned {
            // Could not assign a room - hotel is overbooked
            return None;
        }
    }

    Some(assignments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::BookingStatus;

    fn fake_booking(id: i64, start_day: u32, end_day: u32) -> Booking {
        Booking {
            id,
            hotel_id: 1,
            room_number: None,
            guest_name: format!("Guest {}", id),
            start_time: NaiveDate::from_ymd_opt(2024, 1, start_day).unwrap(),
            end_time: NaiveDate::from_ymd_opt(2024, 1, end_day).unwrap(),
            status: BookingStatus::Confirmed,
        }
    }

    fn request_booking(start_day: u32, end_day: u32) -> (NaiveDate, NaiveDate) {
        (
            NaiveDate::from_ymd_opt(2024, 1, start_day).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, end_day).unwrap(),
        )
    }

    #[test]
    fn test_can_accommodate_single_booking() {
        let existing_bookings = vec![];
        let (start, end) = request_booking(1, 3);
        
        assert!(can_accommodate_booking(1, existing_bookings, start, end));
    }

    #[test]
    fn test_can_accommodate_non_overlapping() {
        let existing_bookings = vec![fake_booking(1, 1, 3)];
        let (start, end) = request_booking(4, 6);
        
        assert!(can_accommodate_booking(1, existing_bookings, start, end));
    }

    #[test]
    fn test_cannot_accommodate_overlapping_single_room() {
        let existing_bookings = vec![fake_booking(1, 1, 5)];
        let (start, end) = request_booking(3, 7);
        
        assert!(!can_accommodate_booking(1, existing_bookings, start, end));
    }

    #[test]
    fn test_can_accommodate_overlapping_multiple_rooms() {
        let existing_bookings = vec![fake_booking(1, 1, 5)];
        let (start, end) = request_booking(3, 7);
        
        assert!(can_accommodate_booking(2, existing_bookings, start, end));
    }

    #[test]
    fn test_complex_multiple_overlapping_bookings() {
        // Scenario: 3-room hotel with multiple existing bookings
        let existing_bookings = vec![
            fake_booking(1, 1, 5),   // Room 1: Jan 1-5
            fake_booking(2, 3, 8),   // Room 2: Jan 3-8  
            fake_booking(3, 6, 10),  // Room 3: Jan 6-10
            fake_booking(4, 12, 15), // Room 1: Jan 12-15
        ];
        
        // Should fit in remaining room during Jan 7-9
        let (start, end) = request_booking(7, 9);
        assert!(can_accommodate_booking(3, existing_bookings.clone(), start, end));

        // Should NOT fit - all rooms occupied during Jan 7-9
        assert!(!can_accommodate_booking(2, existing_bookings, start, end));
    }

    #[test]
    fn test_sequential_same_room_bookings() {
        // Back-to-back bookings should work in same room
        let existing_bookings = vec![
            fake_booking(1, 1, 5),   // Jan 1-5
            fake_booking(2, 5, 10),  // Jan 5-10 (checkout/checkin same day)
        ];
        
        // New booking Jan 10-15 should work (consecutive)
        let (start, end) = request_booking(10, 15);
        assert!(can_accommodate_booking(1, existing_bookings, start, end));
    }

    #[test]
    fn test_maximum_capacity_reached() {
        // Fill all rooms completely for overlapping period
        let existing_bookings = vec![
            fake_booking(1, 1, 10),  // Room 1: Jan 1-10
            fake_booking(2, 2, 9),   // Room 2: Jan 2-9
            fake_booking(3, 3, 8),   // Room 3: Jan 3-8
        ];
        
        // Any booking overlapping with Jan 3-8 should fail
        let (start, end) = request_booking(5, 7);
        assert!(!can_accommodate_booking(3, existing_bookings, start, end));
    }

    #[test]
    fn test_interleaved_bookings() {
        // Complex interleaved pattern - let's trace through this manually:
        // Room 1: Jan 1-3, then Jan 4-6 (booking 4)
        // Room 2: Jan 2-4, then Jan 5-7 (booking 5)  
        // Room 3: Jan 3-5
        let existing_bookings = vec![
            fake_booking(1, 1, 3),   // Room 1: Jan 1-3
            fake_booking(2, 2, 4),   // Room 2: Jan 2-4
            fake_booking(3, 3, 5),   // Room 3: Jan 3-5
            fake_booking(4, 4, 6),   // Room 1 free after Jan 3: Jan 4-6
            fake_booking(5, 5, 7),   // Room 2 free after Jan 4: Jan 5-7
        ];
        
        // New booking Jan 6-8: 
        // Room 1 free after Jan 6, Room 2 free after Jan 7, Room 3 free after Jan 5
        // Should fit in Room 3 (free after Jan 5) with 3 rooms
        let (start, end) = request_booking(6, 8);
        assert!(can_accommodate_booking(3, existing_bookings.clone(), start, end));

        // With only 2 rooms, let's check if it can still fit
        // Actually, since Room 3 wouldn't exist with 2 rooms, booking 3 would go to Room 1 or 2
        // This makes the scheduling different, so the test should be adjusted
        let existing_bookings_2room = vec![
            fake_booking(1, 1, 3),   // Room 1: Jan 1-3
            fake_booking(2, 2, 4),   // Room 2: Jan 2-4
            fake_booking(3, 4, 5),   // Room 1: Jan 4-5 (room 1 available after Jan 3)
            fake_booking(4, 5, 6),   // Room 2: Jan 5-6 (room 2 available after Jan 4) 
            fake_booking(5, 6, 7),   // Room 1: Jan 6-7 (room 1 available after Jan 5)
        ];
        
        // New booking Jan 8-10 should work (both rooms free after Jan 7)
        let (start, end) = request_booking(8, 10);
        assert!(can_accommodate_booking(2, existing_bookings_2room, start, end));
    }

    #[test]
    fn test_long_stay_blocks_multiple_short_stays() {
        let existing_bookings = vec![
            fake_booking(1, 1, 30),  // Long stay: entire month
        ];
        
        // Multiple short bookings during the long stay should fail
        let (start, end) = request_booking(5, 7);
        assert!(!can_accommodate_booking(1, existing_bookings.clone(), start, end));
        
        let (start, end) = request_booking(15, 17);
        assert!(!can_accommodate_booking(1, existing_bookings.clone(), start, end));
        
        // But should work with 2 rooms
        let (start, end) = request_booking(10, 15);
        assert!(can_accommodate_booking(2, existing_bookings, start, end));
    }

    #[test]
    fn test_edge_case_same_day_checkin_checkout() {
        let existing_bookings = vec![
            fake_booking(1, 1, 5),   // Jan 1-5 (checkout 11am Jan 5)
        ];
        
        // New booking starting same day as checkout should work
        let start = NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(); // checkin Jan 5
        let end = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        
        assert!(can_accommodate_booking(1, existing_bookings, start, end));
    }
}