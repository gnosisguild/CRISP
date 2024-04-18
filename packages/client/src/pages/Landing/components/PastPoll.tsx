import React from 'react'
import PollCard from '@/components/Cards/PollCard'
import { PAST_POLLS } from '@/mocks/polls'
import { PollResult } from '@/model/poll.model'

type PastPollSectionProps = {
  customLabel?: string
}
const PastPollSection: React.FC<PastPollSectionProps> = ({ customLabel = 'Past polls' }) => {
  return (
    <div className={`flex min-h-screen w-screen flex-col items-center justify-center space-y-12 px-6 py-32`}>
      <h1 className='text-h1 font-bold text-slate-600'>{customLabel}</h1>
      <div className='flex w-full flex-wrap justify-center gap-16 md:gap-8'>
        {PAST_POLLS.map(({ totalVotes, options, roundId, date }: PollResult) => (
          <PollCard key={roundId} pollOptions={options} totalVotes={totalVotes} date={date} />
        ))}
      </div>
      <button className='button-outlined button-max'>view all polls</button>
    </div>
  )
}

export default PastPollSection
