import { PollOption, PollRequestResult, PollResult } from '@/model/poll.model'

export const markWinner = (options: PollOption[]) => {
  const highestVoteCount = Math.max(...options.map((o) => o.votes))
  return options.map((option) => ({
    ...option,
    checked: option.votes === highestVoteCount,
  }))
}

export const convertTimestampToDate = (timestamp: number, secondsToAdd: number = 0): Date => {
  const date = new Date(timestamp * 1000)
  date.setSeconds(date.getMinutes() + secondsToAdd)
  return date
}

export const formatDate = (isoDateString: string): string => {
  const date = new Date(isoDateString)

  const formatter = new Intl.DateTimeFormat('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })

  return formatter.format(date)
}

export const fixPollResult = (poll: PollRequestResult): PollRequestResult => {
  let fixedPollResult = { ...poll }
  fixedPollResult.option_1_tally = poll.option_2_tally
  fixedPollResult.option_2_tally = poll.option_1_tally
  return fixedPollResult
}

export const convertPollData = (request: PollRequestResult): PollResult => {
  const totalVotes = request.option_1_tally + request.option_2_tally
  const options: PollOption[] = [
    {
      value: 0,
      votes: request.option_1_tally,
      label: request.option_1_emoji,
      checked: false,
    },
    {
      value: 1,
      votes: request.option_2_tally,
      label: request.option_2_emoji,
      checked: false,
    },
  ]

  const date = new Date(request.end_time * 1000).toISOString()

  return {
    roundId: request.round_id,
    totalVotes: totalVotes,
    date: date,
    options: options,
  }
}
