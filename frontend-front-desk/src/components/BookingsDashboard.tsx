import { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'

interface Booking {
  id: number
  hotel_id: number
  room_number: number | null
  guest_name: string
  start_time: string
  end_time: string
  status: string
}

interface Hotel {
  id: number
  name: string
  room_count: number
}

export default function BookingsDashboard() {
  const { hotelId } = useParams<{ hotelId: string }>()
  const [bookings, setBookings] = useState<Booking[]>([])
  const [hotel, setHotel] = useState<Hotel | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const [checkinLoading, setCheckinLoading] = useState<number | null>(null)

  const today = new Date().toISOString().split('T')[0]

  const loadHotel = async () => {
    if (!hotelId) return
    
    try {
      const response = await fetch(`http://localhost:3000/hotels/${hotelId}`)
      if (response.ok) {
        const data = await response.json()
        setHotel(data)
      } else {
        setError('Hotel not found')
      }
    } catch (error) {
      setError('Failed to load hotel')
      console.error('Failed to load hotel:', error)
    }
  }

  const loadBookings = async () => {
    if (!hotelId) return

    try {
      setLoading(true)
      const response = await fetch(`http://localhost:3000/hotels/${hotelId}/bookings?date=${today}`)
      if (response.ok) {
        const data = await response.json()
        setBookings(data)
      } else {
        setError('Failed to load bookings')
      }
    } catch (error) {
      setError('Failed to connect to server')
      console.error('Failed to load bookings:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleCheckin = async (bookingId: number) => {
    setCheckinLoading(bookingId)
    
    try {
      const response = await fetch(`http://localhost:3000/bookings/${bookingId}/checkin?today=${today}`, {
        method: 'POST',
      })
      
      if (response.ok) {
        // Re-fetch bookings to get updated status
        await loadBookings()
      } else {
        const errorData = await response.json()
        setError(`Failed to check in: ${errorData.error || 'Unknown error'}`)
      }
    } catch (error) {
      setError('Failed to check in booking')
      console.error('Checkin failed:', error)
    } finally {
      setCheckinLoading(null)
    }
  }

  useEffect(() => {
    loadHotel()
    loadBookings()
  }, [hotelId])

  const formatDate = (dateStr: string) => {
    return new Date(dateStr).toLocaleDateString()
  }

  const getStatusBadge = (status: string) => {
    const statusClass = status.toLowerCase().replace('_', '-')
    return <span className={`status-badge ${statusClass}`}>{status}</span>
  }

  if (loading) {
    return <div className="loading">Loading bookings...</div>
  }

  if (error) {
    return (
      <div className="error-container">
        <div className="error">Error: {error}</div>
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

      <div className="bookings-container">
        <h3>Today's Bookings ({bookings.length})</h3>
        
        {bookings.length === 0 ? (
          <div className="empty-state">
            <p>No bookings for today</p>
          </div>
        ) : (
          <div className="bookings-list">
            {bookings.map(booking => (
              <div key={booking.id} className="booking-card">
                <div className="booking-header">
                  <div className="booking-header-left">
                    <h4>{booking.guest_name}</h4>
                    {getStatusBadge(booking.status)}
                  </div>
                  {booking.status === 'confirmed' && (
                    <button
                      className="checkin-button"
                      onClick={() => handleCheckin(booking.id)}
                      disabled={checkinLoading === booking.id}
                    >
                      {checkinLoading === booking.id ? 'Checking in...' : 'Check In'}
                    </button>
                  )}
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
    </div>
  )
}