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

  // Multi-layered offline detection: Browser events + electric stream state polling
  useEffect(() => {
    // Layer 1: Browser online/offline events for immediate detection
    const handleBrowserOffline = () => {
      setOffline(true)
    }

    const handleBrowserOnline = () => {
      // Trigger faster reconnection check when browser comes online
      stream.forceDisconnectAndRefresh();
    }

    window.addEventListener('offline', handleBrowserOffline)
    window.addEventListener('online', handleBrowserOnline)

    // Layer 2: Fallback connection state monitoring
    const checkConnectionState = () => {
      if (!navigator.onLine) {
        // If browser says we're offline, trust it immediately
        setOffline(true)
      } else {
        // Otherwise check Electric's connection state
        const isConnected = stream?.isConnected() ?? false
        setOffline(!isConnected)
      }
    }
    const interval = setInterval(checkConnectionState, 500)

    return () => {
      window.removeEventListener('offline', handleBrowserOffline)
      window.removeEventListener('online', handleBrowserOnline)
      clearInterval(interval)
    }
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