import React, { useState } from 'react'
import Card from '../../../components/Card'
import { DAILY_POLL } from '../../../mocks/polls'
import { Poll } from '../../../model/poll.model'

type DailyPollSectionProps = {
  isScreen?: boolean
}

const DailyPollSection: React.FC<DailyPollSectionProps> = ({ isScreen }) => {
  const [pollOptions, setPollOptions] = useState<Poll[]>(DAILY_POLL)

  const handleChecked = (selectedId: number) => {
    const updatedOptions = pollOptions.map((option) => ({
      ...option,
      checked: option.id === selectedId,
    }))

    setPollOptions(updatedOptions)
  }

  return (
    <div
      className={`flex w-screen flex-col items-center justify-center space-y-12 ${!isScreen ? 'h-screen  border-y-2 border-twilight-blue-200' : 'h-screen-minus-header-footer'} `}
    >
      <div className='space-y-2'>
        <p className='text-center text-sm font-extrabold uppercase text-zinc-900'>Daily Poll</p>
        <h1 className='text-h1 font-bold text-twilight-blue-900'>Choose your favorite</h1>
      </div>
      <div className='flex space-x-8'>
        {pollOptions.map((poll) => (
          <Card key={poll.id} checked={poll.checked} onChecked={() => handleChecked(poll.id)}>
            <p className='inline-block text-8xl leading-none'>{poll.label}</p>
          </Card>
        ))}
      </div>
      <button className='button-outlined button-max'>cast vote</button>
    </div>
  )
}

export default DailyPollSection
