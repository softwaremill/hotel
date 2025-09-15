import React, { createContext, useContext, useState, useEffect, useRef } from 'react'
import type { ReactNode } from 'react'

export interface OfflineCheckinEvent {
  bookingId: string
  roomNumber: number
  timestamp: number
  hotelId: string
  today: string
}

interface OfflineEventsContextType {
  pendingEvents: OfflineCheckinEvent[]
  addCheckinEvent: (bookingId: string, roomNumber: number, hotelId: string, today: string) => void
}

const OfflineEventsContext = createContext<OfflineEventsContextType | undefined>(undefined)

// React Refresh prefers files to export either only components OR only hooks/utilities.
// This file exports both OfflineEventsProvider (component) and useOfflineEvents (hook).
// eslint-disable-next-line react-refresh/only-export-components
export const useOfflineEvents = () => {
  const context = useContext(OfflineEventsContext)
  if (!context) {
    throw new Error('useOfflineEvents must be used within an OfflineEventsProvider')
  }
  return context
}

interface OfflineEventsProviderProps {
  children: ReactNode
}

const STORAGE_KEY = 'hotel-offline-events'

export const OfflineEventsProvider: React.FC<OfflineEventsProviderProps> = ({ children }) => {
  const pendingEventsRef = useRef<OfflineCheckinEvent[]>([])
  const syncRunningRef = useRef(false)
  const [, forceUpdate] = useState({})

  const triggerRerender = () => forceUpdate({})

  // Load events from localStorage on mount
  useEffect(() => {
    const stored = localStorage.getItem(STORAGE_KEY)
    if (stored) {
      try {
        const events = JSON.parse(stored) as OfflineCheckinEvent[]
        pendingEventsRef.current = events
        triggerRerender()
      } catch (error) {
        console.error('Failed to parse stored offline events:', error)
        localStorage.removeItem(STORAGE_KEY)
      }
    }
  }, [])

  const saveToStorage = () => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(pendingEventsRef.current))
  }

  const addCheckinEvent = (bookingId: string, roomNumber: number, hotelId: string, today: string) => {
    const event: OfflineCheckinEvent = {
      bookingId,
      roomNumber,
      timestamp: Date.now(),
      hotelId,
      today
    }

    pendingEventsRef.current = [...pendingEventsRef.current, event]
    saveToStorage()
    triggerRerender()
  }

  const syncPendingEvents = async () => {
    // Check if sync is already running
    if (syncRunningRef.current) return
    
    // Get current events from ref (always fresh)
    const currentEvents = pendingEventsRef.current
    if (currentEvents.length === 0) return

    // Set sync running flag
    syncRunningRef.current = true

    try {
      // Get the first event to sync
      const event = currentEvents[0]

      // Send client event to the generic client events endpoint
      const clientEvent = {
        type: 'offline_checkin',
        booking_id: event.bookingId,
        room_number: event.roomNumber,
        today: event.today
      }

      const response = await fetch(`http://localhost:3000/client-events`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(clientEvent)
      })

      if (response.ok || (response.status >= 400 && response.status < 500)) {
        // Remove event on success (2xx) or client error (4xx)
        pendingEventsRef.current = pendingEventsRef.current.slice(1)
        saveToStorage()
        triggerRerender()

        if (response.ok) {
          console.log(`Successfully synced offline checkin for booking ${event.bookingId}`)
        } else {
          console.warn(`Client error syncing event for booking ${event.bookingId}, removed from queue:`, await response.text())
        }
      } else {
        // 5xx server error - keep event for retry
        console.error(`Server error syncing event for booking ${event.bookingId}, will retry:`, await response.text())
      }
    } catch (error) {
      console.error(`Network error syncing event:`, error)
      // Keep the event for retry later
    } finally {
      syncRunningRef.current = false
    }
  }

  // Automatic periodic sync - check every second
  useEffect(() => {
    const syncInterval = setInterval(() => {
      syncPendingEvents().catch(console.error)
    }, 1000)

    return () => clearInterval(syncInterval)
  }, []) // No dependencies - function doesn't capture state

  return (
    <OfflineEventsContext.Provider value={{
      pendingEvents: pendingEventsRef.current,
      addCheckinEvent
    }}>
      {children}
    </OfflineEventsContext.Provider>
  )
}