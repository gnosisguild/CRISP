import { ReactNode } from 'react'
import * as WasmInstance from 'libs/wasm/pkg/crisp_web'
import { SocialAuth } from '@/model/twitter.model'
import { BroadcastVoteRequest, BroadcastVoteResponse, VotingRound, VotingTime } from '@/model/vote.model'

export type VoteManagementContextType = {
  isLoading: boolean
  wasmInstance: WasmInstance.InitOutput | null
  encryptInstance: WasmInstance.Encrypt | null
  user: SocialAuth | null
  votingRound: VotingRound | null
  roundEndDate: Date | null
  initialLoad: () => Promise<void>
  setVotingRound: React.Dispatch<React.SetStateAction<VotingRound | null>>
  setUser: (value: SocialAuth | null) => void
  initWebAssembly: () => Promise<void>
  encryptVote: (voteId: bigint, publicKey: Uint8Array) => Promise<Uint8Array | undefined>
  getPkByRound: (round: VotingRound) => Promise<VotingRound | undefined>
  broadcastVote: (vote: BroadcastVoteRequest) => Promise<BroadcastVoteResponse | undefined>
  getStartTimeByRound: (votingStart: VotingTime) => Promise<void>
  logout: () => void
}

export type VoteManagementProviderProps = {
  children: ReactNode
}
