import React, { useEffect, useState } from 'react'
import Card from './Card'
import { PollOption } from '../model/poll.model'

interface PollCardProps {
  pollOptions: PollOption[]
  totalVotes: number
  date: string
}

const PollCard: React.FC<PollCardProps> = ({ pollOptions, totalVotes, date }) => {
  const [results, setResults] = useState<PollOption[]>(pollOptions)

  const markWinner = (options: PollOption[]) => {
    const highestVoteCount = Math.max(...options.map((o) => o.votes))
    return options.map((option) => ({
      ...option,
      checked: option.votes === highestVoteCount,
    }))
  }

  useEffect(() => {
    const newPollOptions = markWinner(pollOptions)
    setResults(newPollOptions)
  }, [pollOptions])

  const calculatePercentage = (votes: number) => {
    return ((votes / totalVotes) * 100).toFixed(0)
  }

  return (
    <div className='relative flex w-full cursor-pointer flex-col items-center justify-center space-y-4 rounded-3xl border-2 border-twilight-blue-200 bg-white-500 p-8 pt-2 shadow-lg md:max-w-[274px]'>
      <div className='external-icon absolute right-4 top-4' />
      <div className='text-xs font-bold text-twilight-blue-900'>{date}</div>
      <div className='flex space-x-8 '>
        {results.map((poll) => (
          <div className='flex flex-col items-center justify-center space-y-4' key={poll.id}>
            <div className='h-[80px] w-[80px]'>
              <Card isDetails checked={poll.checked}>
                <p className='text-5xl'>{poll.label}</p>
              </Card>
            </div>
            <div>
              <h3 className={`text-center text-h3 font-bold ${poll.checked ? 'text-green-light' : 'text-twilight-blue-500'}`}>
                {calculatePercentage(poll.votes)}%
              </h3>
              <p className='text-center text-xs font-bold text-twilight-blue-500'>{poll.votes} votes</p>
            </div>
          </div>
        ))}
      </div>

      <div className='absolute bottom-[-1rem] left-1/2 -translate-x-1/2 transform rounded-lg border-2 border-twilight-blue-200 bg-white-900 p-2 py-1 text-center font-bold text-zinc-500 shadow-md'>
        {totalVotes} VOTES
      </div>
    </div>
  )
}

export default PollCard
