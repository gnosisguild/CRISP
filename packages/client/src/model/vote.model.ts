export interface VotingConfigRequest {
  round_id: number
  chain_id: number
  voting_address: string
  ciphernode_count: number
  voter_count: number
}

export interface VotingRound {
  round_id: number
  pk_bytes: number[]
}

export interface RoundCount {
  round_count: number
}

export interface BroadcastVoteRequest {
  round_id: number
  enc_vote_bytes: number[] //bytes
}

export interface BroadcastVoteResponse {
  response: string
  tx_hash: string
}
