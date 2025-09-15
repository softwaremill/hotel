import { useState } from 'react'
import type { Booking } from '../hooks/useElectricBookings'

interface RoomSelectorProps {
  booking: Booking
  hotelRoomCount: number
  occupiedRooms: Set<number>
  onRoomSelect: (roomNumber: number) => void
  onCancel: () => void
}

export default function RoomSelector({ 
  booking, 
  hotelRoomCount, 
  occupiedRooms, 
  onRoomSelect, 
  onCancel 
}: RoomSelectorProps) {
  const [selectedRoom, setSelectedRoom] = useState<number | null>(null)

  // Generate available room numbers
  const availableRooms = []
  for (let i = 1; i <= hotelRoomCount; i++) {
    if (!occupiedRooms.has(i)) {
      availableRooms.push(i)
    }
  }

  const handleConfirm = () => {
    if (selectedRoom) {
      onRoomSelect(selectedRoom)
    }
  }

  return (
    <div className="room-selector-overlay">
      <div className="room-selector-modal">
        <div className="room-selector-header">
          <h3>Manual Check-in (Offline Mode)</h3>
          <p>Guest: <strong>{booking.guest_name}</strong></p>
          <p>Booking ID: <strong>{booking.id}</strong></p>
        </div>

        <div className="room-selector-content">
          <p>Select an available room:</p>
          
          {availableRooms.length === 0 ? (
            <div className="no-rooms-message">
              <p>No available rooms for check-in.</p>
            </div>
          ) : (
            <div className="room-grid">
              {availableRooms.map(roomNumber => (
                <button
                  key={roomNumber}
                  className={`room-option ${selectedRoom === roomNumber ? 'selected' : ''}`}
                  onClick={() => setSelectedRoom(roomNumber)}
                >
                  Room {roomNumber}
                </button>
              ))}
            </div>
          )}
        </div>

        <div className="room-selector-actions">
          <button className="cancel-button" onClick={onCancel}>
            Cancel
          </button>
          <button 
            className="confirm-button" 
            onClick={handleConfirm}
            disabled={!selectedRoom}
          >
            Check In to Room {selectedRoom || ''}
          </button>
        </div>
      </div>
    </div>
  )
}