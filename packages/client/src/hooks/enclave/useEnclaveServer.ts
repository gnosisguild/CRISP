import { handleGenericError } from '@/utils/handle-generic-error'
import { BroadcastVoteRequest, BroadcastVoteResponse, RoundCount, VoteStateLite } from '@/model/vote.model'
import { useApi } from '../generic/useFetchApi'
import { PollRequestResult } from '@/model/poll.model'
import { fixPollResult, fixResult } from '@/utils/methods'
import { Auth } from '@/model/auth.model'


const ENCLAVE_API = import.meta.env.VITE_ENCLAVE_API

if (!ENCLAVE_API) handleGenericError('useEnclaveServer', { name: 'ENCLAVE_API', message: 'Missing env VITE_ENCLAVE_API' })

const EnclaveEndpoints = {
  GetRound: `${ENCLAVE_API}/get_rounds`,
  GetRoundStateLite: `${ENCLAVE_API}/get_round_state_lite`,
  GetWebResult: `${ENCLAVE_API}/get_web_result`,
  GetWebAllResult: `${ENCLAVE_API}/get_web_result_all`,
  BroadcastVote: `${ENCLAVE_API}/broadcast_enc_vote`,
  Authentication: `${ENCLAVE_API}/authentication_login`,
} as const

export const useEnclaveServer = () => {
  const { GetRound, GetWebAllResult, BroadcastVote, GetRoundStateLite, Authentication, GetWebResult } = EnclaveEndpoints
  const { fetchData, isLoading } = useApi()
  const getRound = () => fetchData<RoundCount>(GetRound)
  const getToken = (postId: string) => fetchData<Auth, { postId: string }>(Authentication, 'post', { postId })
  const getRoundStateLite = (round_id: number) => fetchData<VoteStateLite, { round_id: number }>(GetRoundStateLite, 'post', { round_id })
  const broadcastVote = (vote: BroadcastVoteRequest) => fetchData<BroadcastVoteResponse, BroadcastVoteRequest>(BroadcastVote, 'post', vote)
  const getWebResult = async () => {
    const result = await fetchData<{ states: PollRequestResult[] }, void>(GetWebAllResult, 'get')
    if (result) {
      return fixPollResult(result.states)
    }
    return
  }
  const getWebResultByRound = async (round_id: number) => {
    const result = await fetchData<PollRequestResult, { round_id: number }>(GetWebResult, 'post', { round_id })
    if (result) {
      return fixResult(result)
    }
    return
  }

  return {
    isLoading,
    getWebResultByRound,
    getToken,
    getWebResult,
    getRound,
    getRoundStateLite,
    broadcastVote,
  }
}
