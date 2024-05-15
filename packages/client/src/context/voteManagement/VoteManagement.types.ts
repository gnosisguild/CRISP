import { ReactNode } from 'react'
import * as WasmInstance from 'libs/wasm/pkg/crisp_web'
import { SocialAuth } from '@/model/twitter.model'
import { BroadcastVoteRequest, BroadcastVoteResponse, VoteStateLite, VotingRound } from '@/model/vote.model'
import { Poll, PollRequestResult, PollResult } from '@/model/poll.model'

export type VoteManagementContextType = {
  isLoading: boolean
  wasmInstance: WasmInstance.InitOutput | null
  encryptInstance: WasmInstance.Encrypt | null
  user: SocialAuth | null
  votingRound: VotingRound | null
  roundEndDate: Date | null
  pollOptions: Poll[]
  roundState: VoteStateLite | null
  pastPolls: PollResult[]
  txUrl: string | undefined
  setTxUrl: React.Dispatch<React.SetStateAction<string | undefined>>
  setPollOptions: React.Dispatch<React.SetStateAction<Poll[]>>
  initialLoad: () => Promise<void>
  existNewRound: () => Promise<void>
  getPastPolls: (roundCount: number) => Promise<void>
  setVotingRound: React.Dispatch<React.SetStateAction<VotingRound | null>>
  setUser: (value: SocialAuth | null) => void
  initWebAssembly: () => Promise<void>
  encryptVote: (voteId: bigint, publicKey: Uint8Array) => Promise<Uint8Array | undefined>
  broadcastVote: (vote: BroadcastVoteRequest) => Promise<BroadcastVoteResponse | undefined>
  getRoundStateLite: (roundCount: number) => Promise<void>
  setPastPolls: React.Dispatch<React.SetStateAction<PollResult[]>>
  getWebResult: (round_id: number) => Promise<PollRequestResult | undefined>
  logout: () => void
}

export type VoteManagementProviderProps = {
  children: ReactNode
}
