import { useState, useEffect } from 'react'
import { useNavigate } from 'react-router-dom'

interface Hotel {
  id: number
  name: string
}

export default function HotelSelection() {
  const [hotels, setHotels] = useState<Hotel[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState('')
  const navigate = useNavigate()

  const loadHotels = async () => {
    try {
      setLoading(true)
      const response = await fetch('http://localhost:3000/hotels')
      if (response.ok) {
        const data = await response.json()
        setHotels(data)
      } else {
        setError('Failed to load hotels')
      }
    } catch (error) {
      setError('Failed to connect to server')
      console.error('Failed to load hotels:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleHotelSelect = (hotel: Hotel) => {
    navigate(`/hotel/${hotel.id}`)
  }

  useEffect(() => {
    loadHotels()
  }, [])

  if (loading) {
    return <div className="loading">Loading hotels...</div>
  }

  if (error) {
    return <div className="error">Error: {error}</div>
  }

  return (
    <div className="hotel-selection">
      <h2>Select Hotel</h2>
      <p className="subtitle">Choose the hotel you are managing front desk for:</p>
      
      <div className="hotels-grid">
        {hotels.map(hotel => (
          <div
            key={hotel.id}
            className="hotel-card"
            onClick={() => handleHotelSelect(hotel)}
          >
            <h3>{hotel.name}</h3>
            <p>Click to manage</p>
          </div>
        ))}
      </div>

      {hotels.length === 0 && (
        <p className="empty-state">No hotels found</p>
      )}
    </div>
  )
}