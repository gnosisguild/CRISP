import React from 'react'
import PollCard from '../../../components/PollCard'
import { PAST_POLLS } from '../../../mocks/polls'
import { PollResult } from '../../../model/poll.model'

const PastPollSection: React.FC = () => {
  return (
    <div className='flex w-screen flex-col items-center justify-center space-y-12' style={{ height: 'calc(100vh - 16px)' }}>
      <h1 className='text-h1 font-bold text-twilight-blue-900'>Past polls</h1>
      <div className='flex w-full flex-wrap justify-center gap-8'>
        {PAST_POLLS.map(({ totalVotes, options, id, date }: PollResult) => (
          <PollCard key={id} pollOptions={options} totalVotes={totalVotes} date={date} />
        ))}
      </div>
      <button className='button-outlined button-max'>view all polls</button>
    </div>
  )
}

export default PastPollSection
