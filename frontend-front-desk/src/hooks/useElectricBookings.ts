import { useShape } from '@electric-sql/react'
import { useMemo, useEffect } from 'react'
import { useOffline } from '../contexts/OfflineContext'

export interface Booking extends Record<string, unknown> {
  id: number
  hotel_id: number
  room_number: number | null
  guest_name: string
  start_time: string
  end_time: string
  status: string
}

export function useElectricBookings(hotelId: string, today: string) {
  const { setOffline } = useOffline()

  const { isLoading, data, error, stream } = useShape<Booking>({
    url: `http://localhost:3000/hotels/${hotelId}/bookings/shape?date=${today}`,
  })

  // Monitor connection state using Electric's internal state
  useEffect(() => {
    const checkConnectionState = () => {
      const isConnected = stream?.isConnected() ?? false
      setOffline(!isConnected)
    }

    // Check connection state periodically
    const interval = setInterval(checkConnectionState, 100)
    return () => clearInterval(interval)
  }, [stream, setOffline])

  const bookings = useMemo(() => {
    if (!data) return []

    // Server-side filtering is now handled by the backend
    // Just sort alphabetically by guest name  
    return [...data].sort((a, b) =>
      String(a.guest_name).localeCompare(String(b.guest_name))
    )
  }, [data])

  return {
    bookings,
    loading: isLoading,
    error: error ? String(error) : null,
  }
}