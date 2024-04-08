import React, { useEffect, useState } from 'react'
import { PollOption } from '../../model/poll.model'
import VotesBadge from '../VotesBadge'
import { useNavigate } from 'react-router-dom'
import PollCardResult from './PollCardResult'
import { markWinner } from '../../utils/methods'

interface PollCardProps {
  pollOptions: PollOption[]
  totalVotes: number
  date: string
}

const PollCard: React.FC<PollCardProps> = ({ pollOptions, totalVotes, date }) => {
  const navigate = useNavigate()
  const [results, setResults] = useState<PollOption[]>(pollOptions)

  useEffect(() => {
    const newPollOptions = markWinner(pollOptions)
    setResults(newPollOptions)
  }, [pollOptions])

  const handleNavigation = () => {
    navigate('/result')
  }

  return (
    <div
      className='relative flex w-full cursor-pointer flex-col items-center justify-center space-y-4 rounded-3xl border-2 border-twilight-blue-200 bg-white-500 p-8 pt-2 shadow-lg md:max-w-[274px]'
      onClick={handleNavigation}
    >
      <div className='external-icon absolute right-4 top-4' />
      <div className='text-xs font-bold text-twilight-blue-900'>{date}</div>
      <div className='flex space-x-8 '>
        <PollCardResult results={results} totalVotes={totalVotes} />
      </div>

      <div className='absolute bottom-[-1rem] left-1/2 -translate-x-1/2 transform '>
        <VotesBadge totalVotes={totalVotes} />
      </div>
    </div>
  )
}

export default PollCard
