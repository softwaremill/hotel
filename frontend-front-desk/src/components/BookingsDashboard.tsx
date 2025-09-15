import { useState, useEffect, useMemo } from 'react'
import { useParams, Link } from 'react-router-dom'
import { useElectricBookings } from '../hooks/useElectricBookings'
import { useOffline } from '../contexts/OfflineContext'
import { useOfflineEvents } from '../contexts/OfflineEventsContext'
import RoomSelector from './RoomSelector'
import type { Booking } from '../hooks/useElectricBookings'

interface Hotel {
  id: number
  name: string
  room_count: number
}

export default function BookingsDashboard() {
  const { hotelId } = useParams<{ hotelId: string }>()
  const [hotel, setHotel] = useState<Hotel | null>(null)
  const [hotelError, setHotelError] = useState('')
  const [showRoomSelector, setShowRoomSelector] = useState<Booking | null>(null)
  const { isOffline, setOffline } = useOffline()
  const { addCheckinEvent } = useOfflineEvents()

  const today = new Date().toISOString().split('T')[0]

  // Use Electric hook for real-time bookings
  const { bookings, error } = useElectricBookings(hotelId!, today)

  const loadHotel = async () => {
    if (!hotelId) return

    try {
      const response = await fetch(`http://localhost:3000/hotels/${hotelId}`)
      if (response.ok) {
        const data = await response.json()
        // Convert BigInt fields to regular numbers for safe JavaScript handling
        setHotel({
          ...data,
          id: Number(data.id),
          room_count: Number(data.room_count)
        })
      } else {
        setHotelError('Hotel not found')
      }
    } catch (error) {
      setHotelError('Failed to load hotel')
      console.error('Failed to load hotel:', error)
    }
  }

  const handleCheckin = async (bookingId: string) => {
    // If offline, show room selector for manual checkin
    if (isOffline) {
      const booking = bookings.find(b => b.id === bookingId)
      if (booking) {
        setShowRoomSelector(booking)
      }
      return
    }

    // Online checkin (automatic room assignment by backend)
    try {
      const response = await fetch(`http://localhost:3000/bookings/${bookingId}/checkin?today=${today}`, {
        method: 'POST',
      })

      if (response.ok) {
        setOffline(false)
        // Electric will automatically update the UI when the backend processes the change
      } else {
        const errorData = await response.json()
        console.error(`Failed to check in: ${errorData.error || 'Unknown error'}`)
      }
    } catch (error) {
      console.error('Failed to check in booking:', error)
      // Network error means we're offline
      setOffline(true)
    }
  }

  const handleCheckout = async (bookingId: string) => {
    try {
      const response = await fetch(`http://localhost:3000/bookings/${bookingId}/checkout`, {
        method: 'POST',
      })

      if (response.ok) {
        // Electric will automatically update the UI when the backend processes the change
      } else {
        const errorData = await response.json()
        console.error(`Failed to check out: ${errorData.error || 'Unknown error'}`)
      }
    } catch (error) {
      console.error('Failed to check out booking:', error)
      // Network error means we're offline
      setOffline(true)
    }
  }

  const handleCancel = async (bookingId: string) => {
    try {
      const response = await fetch(`http://localhost:3000/bookings/${bookingId}/cancel`, {
        method: 'POST',
      })

      if (response.ok) {
        // Electric will automatically update the UI when the backend processes the change
      } else {
        const errorData = await response.json()
        console.error(`Failed to cancel: ${errorData.error || 'Unknown error'}`)
      }
    } catch (error) {
      console.error('Failed to cancel booking:', error)
      // Network error means we're offline
      setOffline(true)
    }
  }

  useEffect(() => {
    loadHotel()
  }, [hotelId]) // eslint-disable-line react-hooks/exhaustive-deps

  const formatDate = (dateStr: string) => {
    return new Date(dateStr).toLocaleDateString()
  }

  const getStatusBadge = (status: string) => {
    const statusClass = status.toLowerCase().replace('_', '-')
    return <span className={`status-badge ${statusClass}`}>{status}</span>
  }

  // Calculate occupied rooms for offline checkin room selection
  const occupiedRooms = useMemo(() => {
    const occupied = new Set<number>()
    bookings.forEach(booking => {
      if (booking.status === 'checked_in' && booking.room_number) {
        occupied.add(booking.room_number)
      }
    })
    return occupied
  }, [bookings])

  const handleRoomSelection = (roomNumber: number) => {
    if (showRoomSelector) {
      addCheckinEvent(
        showRoomSelector.id, // Already a string for safe handling of large integers
        roomNumber,
        hotelId!,
        today
      )
      setShowRoomSelector(null)
    }
  }

  const handleCancelRoomSelection = () => {
    setShowRoomSelector(null)
  }


  if (error || hotelError) {
    return (
      <div className="error-container">
        <div className="error">Error: {error || hotelError}</div>
        <Link to="/" className="back-link">← Back to Hotel Selection</Link>
      </div>
    )
  }

  return (
    <div className="bookings-dashboard">
      <div className="header">
        <Link to="/" className="back-link">← Back to Hotel Selection</Link>
        <h2>{hotel?.name} - Front Desk Dashboard</h2>
        <p className="date-info">Bookings for today: {new Date(today).toLocaleDateString()}</p>
      </div>

      {isOffline && (
        <div className="offline-banner">
          Network connectivity problem, working in degraded mode. Only manual checkins are possible.
        </div>
      )}

      <div className="bookings-container">
        <h3>Today's Bookings ({bookings.length})</h3>

        {bookings.length === 0 ? (
          <div className="empty-state">
            <p>No bookings for today</p>
          </div>
        ) : (
          <div className="bookings-list">
            {bookings.map(booking => (
              <div key={booking.id} className={`booking-card ${booking._pendingSync ? 'pending-sync' : ''}`}>
                <div className="booking-header">
                  <div className="booking-header-left">
                    <h4>{booking.guest_name}</h4>
                    <div className="status-container">
                      {getStatusBadge(booking.status)}
                      {booking._pendingSync && (
                        <span className="pending-sync-badge">Pending Sync</span>
                      )}
                    </div>
                  </div>
                  <div className="booking-actions">
                    {booking.status === 'confirmed' && (
                      <>
                        <button
                          className="checkin-button"
                          onClick={() => handleCheckin(booking.id)}
                        >
                          Check In
                        </button>
                        <button
                          className="cancel-button"
                          onClick={() => handleCancel(booking.id)}
                          disabled={isOffline}
                        >
                          Cancel
                        </button>
                      </>
                    )}
                    {booking.status === 'checked_in' && (
                      <button
                        className="checkout-button"
                        onClick={() => handleCheckout(booking.id)}
                        disabled={isOffline}
                      >
                        Check Out
                      </button>
                    )}
                  </div>
                </div>
                <div className="booking-details">
                  <div className="detail">
                    <span className="label">Booking ID:</span>
                    <span className="value">{booking.id}</span>
                  </div>
                  <div className="detail">
                    <span className="label">Room:</span>
                    <span className="value">
                      {booking.room_number ? `Room ${booking.room_number}` : 'Not assigned'}
                    </span>
                  </div>
                  <div className="detail">
                    <span className="label">Check-in:</span>
                    <span className="value">{formatDate(booking.start_time)}</span>
                  </div>
                  <div className="detail">
                    <span className="label">Check-out:</span>
                    <span className="value">{formatDate(booking.end_time)}</span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {showRoomSelector && hotel && (
        <RoomSelector
          booking={showRoomSelector}
          hotelRoomCount={hotel.room_count}
          occupiedRooms={occupiedRooms}
          onRoomSelect={handleRoomSelection}
          onCancel={handleCancelRoomSelection}
        />
      )}
    </div>
  )
}