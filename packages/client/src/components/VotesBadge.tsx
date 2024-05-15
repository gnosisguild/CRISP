import React from 'react'

type VotesBadgeProps = {
  totalVotes: number
  isActive?: boolean
}

const VotesBadge: React.FC<VotesBadgeProps> = ({ totalVotes, isActive }) => {
  return (
    <div
      className={` ${isActive ? 'scale-105 border-lime-400' : 'border-slate-600/20'} w-fit rounded-lg border-2 bg-white p-2 py-1 text-center font-bold uppercase text-slate-800/50 shadow-md`}
    >
      {totalVotes} votes
    </div>
  )
}

export default VotesBadge
