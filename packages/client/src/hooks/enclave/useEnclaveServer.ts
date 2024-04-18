import { useState } from 'react'
import axios from 'axios'
import { handleGenericError } from '@/utils/handle-generic-error'
import { BroadcastVoteRequest, BroadcastVoteResponse, RoundCount, VotingRound } from '@/model/vote.model'

const ENCLAVE_API = import.meta.env.VITE_ENCLAVE_API

if (!ENCLAVE_API) handleGenericError('useEnclaveServer', { name: 'ENCLAVE_API', message: 'Missing env VITE_ENCLAVE_API' })

export const useEnclaveServer = () => {
  const [isLoading, setIsLoading] = useState<boolean>(false)

  const getPkByRound = async (round: VotingRound): Promise<VotingRound | undefined> => {
    try {
      setIsLoading(true)
      const result = await axios.post<VotingRound>(`${ENCLAVE_API}/get_pk_by_round`, round)
      if (result.data) {
        return result.data
      }
    } catch (error) {
      handleGenericError('useEnclaveServer - getPkByRound', error as Error)
      return
    } finally {
      setIsLoading(false)
    }
  }

  const getRound = async (): Promise<RoundCount | undefined> => {
    try {
      setIsLoading(true)
      const result = await axios.get<RoundCount>(`${ENCLAVE_API}/get_rounds`)
      if (result.data) {
        return result.data
      }
    } catch (error) {
      handleGenericError('useEnclaveServer - getRound', error as Error)
      return
    } finally {
      setIsLoading(false)
    }
  }

  const broadcastVote = async (vote: BroadcastVoteRequest): Promise<BroadcastVoteResponse | undefined> => {
    try {
      setIsLoading(true)
      const result = await axios.post<BroadcastVoteResponse>(`${ENCLAVE_API}/broadcast_enc_vote`, vote)
      if (result.data) {
        return result.data
      }
    } catch (error) {
      handleGenericError('useEnclaveServer - broadcastVote', error as Error)
      return
    } finally {
      setIsLoading(false)
    }
  }

  return {
    isLoading,
    getPkByRound,
    getRound,
    broadcastVote,
  }
}
