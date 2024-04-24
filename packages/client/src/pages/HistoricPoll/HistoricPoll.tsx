import React, { useEffect } from 'react'
import PollCard from '@/components/Cards/PollCard'
// import { PAST_POLLS } from '@/mocks/polls'
import { PollResult } from '@/model/poll.model'
import LoadingAnimation from '@/components/LoadingAnimation'
import { useVoteManagementContext } from '@/context/voteManagement'

const HistoricPoll: React.FC = () => {
  const { votingRound, pastPolls, getPastPolls } = useVoteManagementContext()

  useEffect(() => {
    if (votingRound && votingRound?.round_id > pastPolls.length) {
      const fetchPastPolls = async () => {
        await getPastPolls(votingRound.round_id)
      }
      fetchPastPolls()
    }
  }, [pastPolls, votingRound])

  return (
    <div className='my-28 flex min-h-[630px] w-screen flex-col items-center justify-center space-y-12'>
      <h1 className='text-h1 font-bold text-slate-600'>Historic polls</h1>
      {!pastPolls.length && (
        <div className='flex justify-center'>
          <LoadingAnimation isLoading={true} />
        </div>
      )}
      <div className='mx-auto grid w-full max-w-7xl grid-cols-1 items-center gap-8 overflow-y-auto p-4 md:grid-cols-3'>
        {pastPolls.map(({ totalVotes, options, roundId, date }: PollResult) => (
          <div className='flex items-center justify-center' key={roundId}>
            <PollCard roundId={roundId} pollOptions={options} totalVotes={totalVotes} date={date} />
          </div>
        ))}
      </div>
      {/* <button className='button-outlined button-max'>view all polls</button> */}
    </div>
  )
}

export default HistoricPoll
