export interface PollOption {
  id: number
  votes: number
  label: string // emoji
  checked?: boolean
}

export interface PollResult {
  id: number
  totalVotes: number
  date: string
  options: PollOption[]
}

export interface Poll {
  id: number
  checked: boolean
  label: string
}
