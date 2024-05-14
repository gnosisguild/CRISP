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
  a: [bigint, bigint]
  b: [[bigint, bigint], [bigint, bigint]]
  c: [bigint, bigint]
}

export interface BroadcastVoteResponse {
  response: string
  tx_hash: string
}

export interface VoteStateLite {
  id: number
  status: string
  poll_length: number
  voting_address: string
  chain_id: number
  ciphernode_count: number
  pk_share_count: number
  sks_share_count: number
  vote_count: number
  crp: number[]
  pk: number[]
  start_time: number
  ciphernode_total: number
  emojis: [string, string]
}
