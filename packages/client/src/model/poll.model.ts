export interface PollOption {
  value: number
  votes: number
  label: string // emoji
  checked?: boolean
}

export interface PollResult {
  roundId: number
  totalVotes: number
  date: string
  options: PollOption[]
}

export interface Poll {
  value: number
  checked: boolean
  label: string
}
