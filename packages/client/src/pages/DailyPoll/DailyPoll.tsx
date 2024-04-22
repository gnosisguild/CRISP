import React, { Fragment, useState } from 'react'
import DailyPollSection from '@/pages/Landing/components/DailyPoll'
import ConfirmVote from '@/pages/DailyPoll/components/ConfirmVote'
import { Poll } from '@/model/poll.model'
import { useVoteManagementContext } from '@/context/voteManagement'
import { useNotificationAlertContext } from '@/context/NotificationAlert'

const DailyPoll: React.FC = () => {
  const { showToast } = useNotificationAlertContext()
  const { encryptVote, broadcastVote, getStartTimeByRound, votingRound, roundEndDate } = useVoteManagementContext()

  const [voteCompleted, setVotedCompleted] = useState<boolean>(false)
  const [loading, setLoading] = useState<boolean>(false)

  const handleVoted = async (vote: Poll | null) => {
    if (vote && votingRound) {
      setLoading(true)
      const voteEncrypted = await encryptVote(BigInt(vote.value), new Uint8Array(votingRound.pk_bytes))
      if (voteEncrypted) {
        const broadcastVoteResponse = await broadcastVote({
          round_id: votingRound.round_id,
          enc_vote_bytes: Array.from(voteEncrypted),
        })
        if (broadcastVoteResponse) {
          showToast({
            type: 'success',
            message: 'Successfully voted',
            linkUrl: `https://sepolia.etherscan.io/tx/${broadcastVoteResponse?.tx_hash}`,
          })
          await getStartTimeByRound({ round_id: votingRound.round_id, timestamp: 0 })
          setVotedCompleted(true)
          return
        }
        showToast({ type: 'danger', message: 'Error broadcasting the vote' })
        setLoading(false)
      }
    }
  }

  return (
    <Fragment>
      {!voteCompleted && <DailyPollSection onVoted={handleVoted} loading={loading} />}
      {voteCompleted && roundEndDate && <ConfirmVote endTime={roundEndDate} />}
    </Fragment>
  )
}

export default DailyPoll
