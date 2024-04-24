import { handleGenericError } from '@/utils/handle-generic-error'
import { BroadcastVoteRequest, BroadcastVoteResponse, RoundCount, VoteStateLite } from '@/model/vote.model'
import { useApi } from '../generic/useFetchApi'
import { PollRequestResult } from '@/model/poll.model'
import { fixPollResult } from '@/utils/methods'

const ENCLAVE_API = import.meta.env.VITE_ENCLAVE_API

if (!ENCLAVE_API) handleGenericError('useEnclaveServer', { name: 'ENCLAVE_API', message: 'Missing env VITE_ENCLAVE_API' })

const EnclaveEndpoints = {
  GetRound: `${ENCLAVE_API}/get_rounds`,
  GetRoundStateLite: `${ENCLAVE_API}/get_round_state_lite`,
  GetWebResult: `${ENCLAVE_API}/get_web_result`,
  BroadcastVote: `${ENCLAVE_API}/broadcast_enc_vote`,
} as const

export const useEnclaveServer = () => {
  const { GetRound, BroadcastVote, GetRoundStateLite, GetWebResult } = EnclaveEndpoints
  const { fetchData, isLoading } = useApi()
  const getRound = () => fetchData<RoundCount>(GetRound)
  const getRoundStateLite = (round_id: number) => fetchData<VoteStateLite, { round_id: number }>(GetRoundStateLite, 'post', { round_id })
  const broadcastVote = (vote: BroadcastVoteRequest) => fetchData<BroadcastVoteResponse, BroadcastVoteRequest>(BroadcastVote, 'post', vote)
  const getWebResult = async (round_id: number) => {
    const result = await fetchData<PollRequestResult, { round_id: number }>(GetWebResult, 'post', { round_id })
    if (result) {
      return fixPollResult(result)
    }
    return
  }

  return {
    isLoading,
    getWebResult,
    getRound,
    getRoundStateLite,
    broadcastVote,
  }
}
