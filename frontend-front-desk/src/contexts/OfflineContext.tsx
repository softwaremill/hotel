import React, { createContext, useContext, useState } from 'react'
import type { ReactNode } from 'react'

interface OfflineContextType {
  isOffline: boolean
  setOffline: (offline: boolean) => void
}

const OfflineContext = createContext<OfflineContextType | undefined>(undefined)

// eslint-disable-next-line react-refresh/only-export-components
export const useOffline = () => {
  const context = useContext(OfflineContext)
  if (!context) {
    throw new Error('useOffline must be used within an OfflineProvider')
  }
  return context
}

interface OfflineProviderProps {
  children: ReactNode
}

export const OfflineProvider: React.FC<OfflineProviderProps> = ({ children }) => {
  const [isOffline, setIsOffline] = useState(false)

  const setOffline = (offline: boolean) => {
    setIsOffline(offline)
  }

  return (
    <OfflineContext.Provider value={{ isOffline, setOffline }}>
      {children}
    </OfflineContext.Provider>
  )
}