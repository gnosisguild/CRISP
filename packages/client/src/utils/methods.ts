import { PollOption } from '@/model/poll.model'

export const markWinner = (options: PollOption[]) => {
  const highestVoteCount = Math.max(...options.map((o) => o.votes))
  return options.map((option) => ({
    ...option,
    checked: option.votes === highestVoteCount,
  }))
}

export const convertTimestampToDate = (timestamp: number): Date => {
  return new Date(timestamp * 1000)
}
