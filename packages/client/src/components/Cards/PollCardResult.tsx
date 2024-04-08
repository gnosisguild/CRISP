import React from 'react'
import { PollOption } from '../../model/poll.model'
import Card from './Card'

type PollCardResultProps = {
  results: PollOption[]
  totalVotes: number
  spaceCards?: string
  height?: number
  width?: number
  isResult?: boolean
}
const PollCardResult: React.FC<PollCardResultProps> = ({
  isResult,
  results,
  totalVotes,
  spaceCards = 'space-x-8',
  height = 80,
  width = 80,
}) => {
  const calculatePercentage = (votes: number) => {
    return ((votes / totalVotes) * 100).toFixed(0)
  }

  return (
    <div className={`flex ${spaceCards}`}>
      {results.map((poll) => (
        <div className={`flex flex-col items-center justify-center ${isResult ? 'space-y-6' : 'space-y-4'}`} key={poll.id}>
          <div className={`h-[${height ?? '80'}] w-[${width ?? '80'}]`} style={{ width, height }}>
            <Card isDetails checked={poll.checked}>
              <p className={isResult ? 'text-8xl' : 'text-5xl'}>{poll.label}</p>
            </Card>
          </div>
          <div className={isResult ? 'space-y-2' : ''}>
            <h3
              className={`text-center ${isResult ? 'text-h1' : 'text-h3'}  font-bold ${poll.checked ? 'text-green-light' : 'text-twilight-blue-500'}`}
            >
              {calculatePercentage(poll.votes)}%
            </h3>
            <p className={`text-center ${isResult ? 'text-2xl font-semibold' : 'text-xs font-bold'}  text-twilight-blue-500`}>
              {poll.votes} votes
            </p>
          </div>
        </div>
      ))}
    </div>
  )
}

export default PollCardResult
