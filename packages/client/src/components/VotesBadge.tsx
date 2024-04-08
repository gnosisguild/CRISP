import React from 'react'

type VotesBadgeProps = {
  totalVotes: number
}

const VotesBadge: React.FC<VotesBadgeProps> = ({ totalVotes }) => {
  return (
    <div className=' w-fit rounded-lg border-2 border-twilight-blue-200 bg-white-900 p-2 py-1 text-center font-bold uppercase text-zinc-500 shadow-md'>
      {totalVotes} votes
    </div>
  )
}

export default VotesBadge
