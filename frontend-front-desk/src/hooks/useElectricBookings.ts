import { useShape } from '@electric-sql/react'
import { useMemo } from 'react'

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
  const { isLoading, data, error } = useShape<Booking>({
    url: `http://localhost:3000/hotels/${hotelId}/bookings/shape?date=${today}`,
  })

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