import React, { Fragment, useEffect } from 'react'
import init, { greet } from '../public/pkg/rfv'
import Navbar from './components/Navbar'
import { Routes, Route } from 'react-router-dom'
import Footer from './components/Footer'
import Landing from './pages/Landing/Landing'
import DailyPoll from './pages/DailyPoll/DailyPoll'
import HistoricPoll from './pages/HistoricPoll/HistoricPoll'

const App: React.FC = () => {
  useEffect(() => {
    init().then(() => {
      greet('World. Consuming web assembly')
    })
  }, [])

  return (
    <Fragment>
      <Navbar />
      <Routes>
        <Route path='/' element={<Landing />} />
        <Route path='/daily' element={<DailyPoll />} />
        <Route path='/historic' element={<HistoricPoll />} />
      </Routes>
      <Footer />
    </Fragment>
  )
}

export default App
