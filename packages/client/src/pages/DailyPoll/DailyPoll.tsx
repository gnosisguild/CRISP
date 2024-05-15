import React, { Fragment, useEffect, useState } from 'react'
import DailyPollSection from '@/pages/Landing/components/DailyPoll'
import { Poll } from '@/model/poll.model'
import { useVoteManagementContext } from '@/context/voteManagement'
import { useNotificationAlertContext } from '@/context/NotificationAlert'
import { useNavigate } from 'react-router-dom'

const DailyPoll: React.FC = () => {
  const navigate = useNavigate()
  const { showToast } = useNotificationAlertContext()
  const { encryptVote, broadcastVote, getRoundStateLite, existNewRound, votingRound, roundState } = useVoteManagementContext()
  const [loading, setLoading] = useState<boolean>(false)
  const [newRoundLoading, setNewRoundLoading] = useState<boolean>(false)

  useEffect(() => {
    const checkRound = async () => {
      setNewRoundLoading(true)
      await existNewRound()
    }
    checkRound()
  }, [])

  useEffect(() => {
    if (roundState) {
      setNewRoundLoading(false)
    }
  }, [roundState])

  const handleVoted = async (vote: Poll | null) => {
    if (vote && votingRound) {
      setLoading(true)
      const voteEncrypted = await encryptVote(BigInt(vote.value), new Uint8Array(votingRound.pk_bytes))

      if (voteEncrypted) {
        const broadcastVoteResponse = await broadcastVote({
          round_id: votingRound.round_id,
          enc_vote_bytes: Array.from(voteEncrypted),
        })
        await getRoundStateLite(votingRound.round_id)
        if (broadcastVoteResponse) {
          showToast({
            type: 'success',
            message: 'Successfully voted',
            linkUrl: `https://sepolia.etherscan.io/tx/${broadcastVoteResponse?.tx_hash}`,
          })
          navigate(`/result/${votingRound.round_id}/confirmation`)
          return
        }
        showToast({ type: 'danger', message: 'Error broadcasting the vote' })
      }
      setLoading(false)
    }
  }
  return (
    <Fragment>
      <DailyPollSection onVoted={handleVoted} loading={loading || newRoundLoading} />
    </Fragment>
  )
}

export default DailyPoll
