import React, { Fragment, useState } from 'react'
import DailyPollSection from '@/pages/Landing/components/DailyPoll'
import ConfirmVote from '@/pages/DailyPoll/components/ConfirmVote'

const DailyPoll: React.FC = () => {
  const [endTime, setEndTime] = useState<Date | null>(null)

  const [voteCompleted, setVotedCompleted] = useState<boolean>(false)

  const handleVoted = () => {
    const newDate = new Date()
    newDate.setHours(newDate.getHours() + 28)
    setEndTime(newDate)
    setVotedCompleted(true)
  }

  return (
    <Fragment>
      {!voteCompleted && <DailyPollSection isScreen onVoted={handleVoted} />}
      {voteCompleted && endTime && <ConfirmVote endTime={endTime} />}
    </Fragment>
  )
}

export default DailyPoll
