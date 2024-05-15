import React, { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { PollOption, PollResult } from '@/model/poll.model'
import VotesBadge from '@/components/VotesBadge'
import PollCardResult from '@/components/Cards/PollCardResult'
import { formatDate, hasPollEndedByTimestamp, markWinner } from '@/utils/methods'

const PollCard: React.FC<PollResult> = ({ roundId, options, totalVotes, date, endTime }) => {
  const navigate = useNavigate()
  const [results, setResults] = useState<PollOption[]>(options)

  const isActive = !hasPollEndedByTimestamp(endTime)

  useEffect(() => {
    const newPollOptions = markWinner(options)
    setResults(newPollOptions)
  }, [options])

  const handleNavigation = () => {
    isActive ? navigate('/current') : navigate(`/result/${roundId}`)
  }

  return (
    <div
      className={`
      ${isActive ? 'scale-105 border-lime-400' : 'border-slate-600/20'}
      relative flex min-h-[248px] w-full cursor-pointer flex-col items-center justify-center space-y-4 rounded-3xl border-2 bg-white/50 p-8 pt-2 shadow-lg md:max-w-[274px]`}
      onClick={handleNavigation}
    >
      <div className='external-icon  absolute right-4 top-4' />
      <div className='text-xs font-bold text-slate-600'>{formatDate(date)}</div>
      <div className='flex space-x-8 '>
        <PollCardResult results={results} totalVotes={totalVotes} isActive={isActive} />
      </div>
      {isActive && <h2 className={`text-center text-2xl font-bold  text-slate-600/50`}>Active</h2>}
      <div className='absolute bottom-[-1rem] left-1/2 -translate-x-1/2 transform '>
        <VotesBadge totalVotes={totalVotes} isActive={isActive} />
      </div>
    </div>
  )
}

export default PollCard
