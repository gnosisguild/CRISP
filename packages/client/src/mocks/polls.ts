import { Poll, PollResult } from '../model/poll.model'

export const PAST_POLLS: PollResult[] = [
  {
    id: 1,
    totalVotes: 451,
    date: 'March 25, 2024',
    options: [
      { id: 1, votes: 230, label: '🌮' },
      { id: 2, votes: 221, label: '🍕' },
    ],
  },
  {
    id: 2,
    totalVotes: 100,
    date: 'March 24, 2024',
    options: [
      { id: 2, votes: 49, label: '🍔' },
      { id: 1, votes: 51, label: '🍜' },
    ],
  },
  {
    id: 3,
    totalVotes: 200,
    date: 'March 23, 2024',
    options: [
      { id: 1, votes: 187, label: '🫓' },
      { id: 2, votes: 13, label: '🍣' },
    ],
  },
]

export const DAILY_POLL: Poll[] = [
  { id: 1, label: '🌮', checked: false },
  { id: 2, label: '🍕', checked: false },
]
