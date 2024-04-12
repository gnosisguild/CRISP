import React from 'react'

type VotesBadgeProps = {
  totalVotes: number
}

const VotesBadge: React.FC<VotesBadgeProps> = ({ totalVotes }) => {
  return (
    <div className=' bg-white w-fit rounded-lg border-2 border-slate-600/20 p-2 py-1 text-center font-bold uppercase text-slate-800/50 shadow-md'>
      {totalVotes} votes
    </div>
  )
}

export default VotesBadge
