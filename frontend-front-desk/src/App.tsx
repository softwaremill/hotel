import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import HotelSelection from './components/HotelSelection'
import BookingsDashboard from './components/BookingsDashboard'
import './App.css'

function App() {
  return (
    <Router>
      <div className="app">
        <h1>Hotel Front Desk</h1>
        <Routes>
          <Route path="/" element={<HotelSelection />} />
          <Route path="/hotel/:hotelId" element={<BookingsDashboard />} />
        </Routes>
      </div>
    </Router>
  )
}

export default App