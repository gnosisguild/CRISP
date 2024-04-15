import React, { useState } from 'react'
import Card from '../../../components/Cards/Card'
import { DAILY_POLL } from '../../../mocks/polls'
import { Poll } from '../../../model/poll.model'

type DailyPollSectionProps = {
  isScreen?: boolean
  onVoted?: () => void
}

const DailyPollSection: React.FC<DailyPollSectionProps> = ({ isScreen, onVoted }) => {
  const [pollOptions, setPollOptions] = useState<Poll[]>(DAILY_POLL)
  const [noPollSelected, setPollSelected] = useState<boolean>(true)

  const handleChecked = (selectedId: number) => {
    const updatedOptions = pollOptions.map((option) => ({
      ...option,
      checked: !option.checked && option.id === selectedId,
    }))

    console.log(updatedOptions)
    setPollOptions(updatedOptions)
    setPollSelected(updatedOptions.every((poll) => !poll.checked))
  }

  console.log(noPollSelected)
  return (
    <div
      className={`flex w-screen flex-col items-center justify-center px-6 ${!isScreen ? 'h-screen  border-y-2 border-twilight-blue-200' : 'h-screen-minus-header-footer'} `}
    >
      <div className='mx-auto flex w-full max-w-screen-md flex-col items-center justify-center space-y-12'>
        <div className='space-y-2'>
          <p className='text-center text-sm font-extrabold uppercase text-zinc-900'>Daily Poll</p>
          <h3 className='font-bold text-twilight-blue-900'>Choose your favorite</h3>
        </div>
        <div className='grid w-full grid-cols-2 gap-4 md:gap-8'>
          {pollOptions.map((poll) => (
            <div key={poll.id} className='col-span-2 md:col-span-1'>
              <Card checked={poll.checked} onChecked={() => handleChecked(poll.id)}>
                <p className='inline-block text-8xl leading-none'>{poll.label}</p>
              </Card>
            </div>
          ))}
        </div>
        <button
          className={`button-outlined button-max ${noPollSelected ? 'button-disabled' : ''}`}
          disabled={noPollSelected}
          onClick={() => onVoted && onVoted()}
        >
          cast vote
        </button>
      </div>
    </div>
  )
}

export default DailyPollSection
