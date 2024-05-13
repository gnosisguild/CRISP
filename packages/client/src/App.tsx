import React, { Fragment, useEffect } from 'react'
import { Routes, Route, Navigate } from 'react-router-dom'
import Navbar from '@/components/Navbar'
import Footer from '@/components/Footer'
//Pages
import Landing from '@/pages/Landing/Landing'
import DailyPoll from '@/pages/DailyPoll/DailyPoll'
import HistoricPoll from '@/pages/HistoricPoll/HistoricPoll'
import About from '@/pages/About/About'
import PollResult from '@/pages/PollResult/PollResult'
import useScrollToTop from '@/hooks/generic/useScrollToTop'
import { useVoteManagementContext } from '@/context/voteManagement'
import useCircuit from '@/hooks/wasm/useCircuit'

const App: React.FC = () => {
  useScrollToTop()
  const { initialLoad, wasmInstance } = useVoteManagementContext()
  const { client } = useCircuit()

  console.log('client', client)
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
      <div className='flex min-h-screen flex-col'>
        <Navbar />
        <div className='flex flex-1 flex-col'>
          <Routes>
            <Route path='/' element={<Landing />} />
            <Route path='/about' element={<About />} />
            <Route path='/daily' element={<DailyPoll />} />
            <Route path='/historic' element={<HistoricPoll />} />
            <Route path='/result/:roundId' element={<PollResult />} />
            <Route path='*' element={<Navigate to='/' replace />} />
          </Routes>
        </div>
        <Footer />
      </div>
    </Fragment>
  )
}

export default App
