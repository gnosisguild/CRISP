import React from 'react'
import PollCard from '@/components/Cards/PollCard'
import { PAST_POLLS } from '@/mocks/polls'
import { PollResult } from '@/model/poll.model'

const HistoricPoll: React.FC = () => {
  return (
    <div className='my-28 flex w-screen flex-col items-center justify-center space-y-12'>
      <h1 className='text-h1 font-bold text-slate-600'>Historic polls</h1>
      <div className='mx-auto grid w-full max-w-7xl grid-cols-1 gap-8 overflow-y-auto p-4 md:grid-cols-3'>
        {PAST_POLLS.map(({ totalVotes, options, roundId, date }: PollResult) => (
          <div className='flex items-center justify-center' key={roundId}>
            <PollCard pollOptions={options} totalVotes={totalVotes} date={date} />
          </div>
        ))}
      </div>
      <button className='button-outlined button-max'>view all polls</button>
    </div>
  )
}

export default HistoricPoll
