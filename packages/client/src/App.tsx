import React, { Fragment, useEffect } from 'react'
import { Routes, Route, Navigate } from 'react-router-dom'
import Navbar from '@/components/Navbar'
import Footer from '@/components/Footer'
//Pages
import Landing from '@/pages/Landing/Landing'
import DailyPoll from '@/pages/DailyPoll/DailyPoll'
import HistoricPoll from '@/pages/HistoricPoll/HistoricPoll'
import About from '@/pages/About/About'
import WhitePaper from './pages/Docs/WhitePaper'
import PollResult from '@/pages/PollResult/PollResult'
import useScrollToTop from '@/hooks/generic/useScrollToTop'
import { useVoteManagementContext } from '@/context/voteManagement'

const App: React.FC = () => {
  useScrollToTop()
  const { initialLoad, wasmInstance } = useVoteManagementContext()

  useEffect(() => {
    if (!wasmInstance) {
      async function loadWasm() {
        await initialLoad()
      }
      loadWasm()
    }
  }, [wasmInstance])

  return (
    <Fragment>
      <Navbar />
      <Routes>
        <Route path='/' element={<Landing />} />
        <Route path='/about' element={<About />} />
        <Route path='/whitepaper' element={<WhitePaper />} />
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
