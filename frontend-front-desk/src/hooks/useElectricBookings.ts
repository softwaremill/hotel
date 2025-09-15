import { useShape } from '@electric-sql/react'
import { useMemo, useEffect } from 'react'
import { useOffline } from '../contexts/OfflineContext'
import { useOfflineEvents } from '../contexts/OfflineEventsContext'

export interface Booking extends Record<string, unknown> {
  id: string
  hotel_id: number
  room_number: number | null
  guest_name: string
  start_time: string
  end_time: string
  status: string
  // Flag to indicate this booking has unsynced changes
  // _ because it's a temporary client-side flag
  _pendingSync?: boolean
}

export function useElectricBookings(hotelId: string, today: string) {
  const { setOffline } = useOffline()
  const { pendingEvents } = useOfflineEvents()

  const { data, error, stream } = useShape<Booking>({
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
        setOffline(!isConnected);
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

    // Start with server data and convert numeric IDs to strings
    let bookingsWithEvents = data.map(booking => ({
      ...booking,
      id: String(booking.id) // Convert numeric ID to string for consistency
    }))

    // Apply offline checkin events to overlay local changes
    pendingEvents.forEach(event => {
      if (event.hotelId === hotelId && event.today === today) {
        const bookingIndex = bookingsWithEvents.findIndex(b => b.id === event.bookingId)
        if (bookingIndex !== -1) {
          // Update the booking to reflect the offline checkin
          bookingsWithEvents[bookingIndex] = {
            ...bookingsWithEvents[bookingIndex],
            status: 'checked_in',
            room_number: event.roomNumber,
            _pendingSync: true // Mark as having pending changes
          }
        }
      }
    })

    // Sort: actionable bookings first (confirmed/checked_in), then non-actionable (cancelled/checked_out)
    // Within each group, sort alphabetically by guest name
    return bookingsWithEvents.sort((a, b) => {
      const isActionableA = a.status === 'confirmed' || a.status === 'checked_in'
      const isActionableB = b.status === 'confirmed' || b.status === 'checked_in'
      
      // If one is actionable and the other isn't, actionable comes first
      if (isActionableA && !isActionableB) return -1
      if (!isActionableA && isActionableB) return 1
      
      // If both have same actionability, sort by guest name
      return String(a.guest_name).localeCompare(String(b.guest_name))
    })
  }, [data, pendingEvents, hotelId, today])

  return {
    bookings,
    error: error ? String(error) : null,
  }
}