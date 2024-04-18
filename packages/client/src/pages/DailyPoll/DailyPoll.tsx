import React, { Fragment, useState } from 'react'
import DailyPollSection from '@/pages/Landing/components/DailyPoll'
import ConfirmVote from '@/pages/DailyPoll/components/ConfirmVote'
import { Poll } from '@/model/poll.model'
import { useVoteManagementContext } from '@/context/voteManagement'
import { useNotificationAlertContext } from '@/context/NotificationAlert'

const DailyPoll: React.FC = () => {
  const { showToast } = useNotificationAlertContext()
  const { encryptVote, broadcastVote, votingRound } = useVoteManagementContext()
  const [endTime, setEndTime] = useState<Date | null>(null)
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
          console.log('broadcastVoteResponse', broadcastVoteResponse)
          const newDate = new Date()
          newDate.setHours(newDate.getHours() + 28)
          setEndTime(newDate)
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
      {voteCompleted && endTime && <ConfirmVote endTime={endTime} />}
    </Fragment>
  )
}

export default DailyPoll
