import { handleGenericError } from '@/utils/handle-generic-error'
import { BroadcastVoteRequest, BroadcastVoteResponse, RoundCount, VoteCount, VotingRound, VotingTime } from '@/model/vote.model'
import { useApi } from '../generic/useFetchApi'

const ENCLAVE_API = import.meta.env.VITE_ENCLAVE_API

if (!ENCLAVE_API) handleGenericError('useEnclaveServer', { name: 'ENCLAVE_API', message: 'Missing env VITE_ENCLAVE_API' })

const EnclaveEndpoints = {
  GetVoteCountByRound: `${ENCLAVE_API}/get_vote_count_by_round`,
  GetPkByRound: `${ENCLAVE_API}/get_pk_by_round`,
  GetRound: `${ENCLAVE_API}/get_rounds`,
  GetStartTimeByRound: `${ENCLAVE_API}/get_start_time_by_round`,
  BroadcastVote: `${ENCLAVE_API}/broadcast_enc_vote`,
} as const

export const useEnclaveServer = () => {
  const { GetVoteCountByRound, GetPkByRound, GetRound, BroadcastVote, GetStartTimeByRound } = EnclaveEndpoints
  const { fetchData, isLoading } = useApi()

  const getVoteCountByRound = (round: VoteCount) => fetchData<VoteCount, VoteCount>(GetVoteCountByRound, 'post', round)
  const getPkByRound = (round: VotingRound) => fetchData<VotingRound, VotingRound>(GetPkByRound, 'post', round)
  const getRound = () => fetchData<RoundCount>(GetRound)
  const getStartTimeByRound = (votingStart: VotingTime) => fetchData<VotingTime, VotingTime>(GetStartTimeByRound, 'post', votingStart)
  const broadcastVote = (vote: BroadcastVoteRequest) => fetchData<BroadcastVoteResponse, BroadcastVoteRequest>(BroadcastVote, 'post', vote)

  return {
    isLoading,
    getPkByRound,
    getRound,
    getVoteCountByRound,
    getStartTimeByRound,
    broadcastVote,
  }
}
