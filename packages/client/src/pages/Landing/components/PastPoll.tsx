import React from 'react'
import PollCard from '../../../components/Cards/PollCard'
import { PAST_POLLS } from '../../../mocks/polls'
import { PollResult } from '../../../model/poll.model'

type PastPollSectionProps = {
  customClass?: string
  customLabel?: string
}
const PastPollSection: React.FC<PastPollSectionProps> = ({ customClass = 'h-screen-minus-header', customLabel = 'Past polls' }) => {
  return (
    <div className={`${customClass} flex w-screen flex-col items-center justify-center space-y-12 `}>
      <h1 className='text-h1 font-bold text-twilight-blue-900'>{customLabel}</h1>
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
