import React, { Fragment, useEffect, useState } from 'react'
import DailyPollSection from '@/pages/Landing/components/DailyPoll'
import ConfirmVote from '@/pages/DailyPoll/components/ConfirmVote'
import { Poll } from '@/model/poll.model'
import { useVoteManagementContext } from '@/context/voteManagement'
import { useNotificationAlertContext } from '@/context/NotificationAlert'

const DailyPoll: React.FC = () => {
  const { showToast } = useNotificationAlertContext()
  const { encryptVote, broadcastVote, getRoundStateLite, existNewRound, proveVote, votingRound, roundEndDate, roundState } =
    useVoteManagementContext()
  const [voteCompleted, setVotedCompleted] = useState<boolean>(false)
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
      const zpkProve = await proveVote(vote.value)
      if (voteEncrypted && zpkProve) {
        const broadcastVoteResponse = await broadcastVote({
          round_id: votingRound.round_id,
          enc_vote_bytes: Array.from(voteEncrypted),
          a: zpkProve.a,
          b: zpkProve.b,
          c: zpkProve.c,
        })
        await getRoundStateLite(votingRound.round_id)
        if (broadcastVoteResponse) {
          showToast({
            type: 'success',
            message: 'Successfully voted',
            linkUrl: `https://sepolia.etherscan.io/tx/${broadcastVoteResponse?.tx_hash}`,
          })
          setVotedCompleted(true)
          return
        }
        showToast({ type: 'danger', message: 'Error broadcasting the vote' })
      }
      setLoading(false)
    }
  }
  return (
    <Fragment>
      {!voteCompleted && <DailyPollSection onVoted={handleVoted} loading={loading || newRoundLoading} />}
      {voteCompleted && roundEndDate && <ConfirmVote endTime={roundEndDate} />}
    </Fragment>
  )
}

export default DailyPoll
