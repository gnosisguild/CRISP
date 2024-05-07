import React, { useEffect } from 'react'
import PollCard from '@/components/Cards/PollCard'
import { PollResult } from '@/model/poll.model'
import LoadingAnimation from '@/components/LoadingAnimation'
import { useVoteManagementContext } from '@/context/voteManagement'
import CircularTiles from '@/components/CircularTiles'

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
    <div className='relative mt-8 flex w-full flex-1 items-center justify-center px-6 py-12 md:mt-0'>
      <div className='absolute bottom-0 right-0 grid w-full grid-cols-2 gap-2 max-md:opacity-50 md:w-[70vh]'>
        <CircularTiles count={4} />
      </div>
      <div className='relative mx-auto flex w-full flex-col items-center justify-center space-y-8'>
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
      </div>
    </div>
  )
}

export default HistoricPoll
