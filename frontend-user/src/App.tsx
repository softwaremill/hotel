import { useState, useEffect } from 'react'
import './App.css'

interface Hotel {
  id: number
  name: string
}

function App() {
  const [hotels, setHotels] = useState<Hotel[]>([])
  const [selectedHotel, setSelectedHotel] = useState<Hotel | null>(null)
  const [guestName, setGuestName] = useState('')
  const [startDate, setStartDate] = useState('')
  const [endDate, setEndDate] = useState('')
  const [loading, setLoading] = useState(false)
  const [message, setMessage] = useState('')

  const loadHotels = async () => {
    try {
      const response = await fetch('http://localhost:3000/hotels')
      if (response.ok) {
        const data = await response.json()
        setHotels(data)
      }
    } catch (error) {
      console.error('Failed to load hotels:', error)
    }
  }


  const createBooking = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!selectedHotel || !guestName || !startDate || !endDate) return

    setLoading(true)
    setMessage('')

    try {
      const response = await fetch(`http://localhost:3000/hotels/${selectedHotel.id}/bookings`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          guest_name: guestName,
          start_time: startDate,
          end_time: endDate
        })
      })

      const data = await response.json()

      if (response.ok) {
        setMessage(`Booking created successfully! Booking ID: ${data.booking_id}`)
        setGuestName('')
        setStartDate('')
        setEndDate('')
      } else {
        setMessage(`Error: ${data.error} ${data.code ? `(${data.code})` : ''}`)
      }
    } catch (error) {
      setMessage('Failed to create booking. Please try again.')
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    loadHotels()
  }, [])


  return (
    <div className="app">
      <h1>Hotel Booking System</h1>

      <div className="hotel-selection">
        <h2>Select a Hotel</h2>
        <div className="hotels-grid">
          {hotels.map(hotel => (
            <div
              key={hotel.id}
              className={`hotel-card ${selectedHotel?.id === hotel.id ? 'selected' : ''}`}
              onClick={() => setSelectedHotel(hotel)}
            >
              <h3>{hotel.name}</h3>
            </div>
          ))}
        </div>
      </div>

      {selectedHotel && (
        <div className="booking-section">
          <h2>Book a Room at {selectedHotel.name}</h2>

          <form onSubmit={createBooking} className="booking-form">
            <div className="form-group">
              <label htmlFor="guestName">Guest Name:</label>
              <input
                id="guestName"
                type="text"
                value={guestName}
                onChange={(e) => setGuestName(e.target.value)}
                required
              />
            </div>

            <div className="form-group">
              <label htmlFor="startDate">Check-in Date:</label>
              <input
                id="startDate"
                type="date"
                value={startDate}
                onChange={(e) => setStartDate(e.target.value)}
                required
              />
            </div>

            <div className="form-group">
              <label htmlFor="endDate">Check-out Date:</label>
              <input
                id="endDate"
                type="date"
                value={endDate}
                onChange={(e) => setEndDate(e.target.value)}
                required
              />
            </div>

            <button type="submit" disabled={loading}>
              {loading ? 'Creating Booking...' : 'Create Booking'}
            </button>
          </form>

          {message && (
            <div className={`message ${message.includes('Error') ? 'error' : 'success'}`}>
              {message}
            </div>
          )}
        </div>
      )}
    </div>
  )
}

export default App
