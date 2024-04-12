import React, { Fragment } from 'react'
// import init, { greet } from '../public/pkg/rfv'
import { Routes, Route, Navigate } from 'react-router-dom'
import Navbar from './components/Navbar'
import Footer from './components/Footer'
//Pages
import Landing from './pages/Landing/Landing'
import DailyPoll from './pages/DailyPoll/DailyPoll'
import HistoricPoll from './pages/HistoricPoll/HistoricPoll'
import About from './pages/About/About'
import PollResult from './pages/PollResult/PollResult'
import useScrollToTop from './hooks/useScrollToTop'

const App: React.FC = () => {
  useScrollToTop()
  // useEffect(() => {
  //   init().then(() => {
  //     greet('World. Consuming web assembly')
  //   })
  // }, [])
  return (
    <Fragment>
      <Navbar />
      <Routes>
        <Route path='/' element={<Landing />} />
        <Route path='/about' element={<About />} />
        <Route path='/daily' element={<DailyPoll />} />
        <Route path='/historic' element={<HistoricPoll />} />
        <Route path='/result' element={<PollResult />} />
        <Route path='*' element={<Navigate to='/' replace />} />
      </Routes>
      <Footer />
    </Fragment>
  )
}

export default App
