import { useState } from 'react'
import axios from 'axios'
import { handleGenericError } from '@/utils/handle-generic-error'
import { VotingRound } from '@/model/vote.model'

const CHRYSALIS_API = import.meta.env.VITE_CHRYSALIS_API

if (!CHRYSALIS_API) handleGenericError('useChrysalisServer', { name: 'CHRYSALIS_API', message: 'Missing env VITE_CHRYSALIS_API' })

export const useChrysalisServer = () => {
  const [isLoading, setIsLoading] = useState<boolean>(false)

  const getPkByRound = async (round: VotingRound): Promise<VotingRound | undefined> => {
    try {
      setIsLoading(true)
      const result = await axios.post<VotingRound>(`${CHRYSALIS_API}/get_pk_by_round`, round)
      if (result.data) {
        return result.data
      }
    } catch (error) {
      handleGenericError('getPkByRound', error as Error)
    } finally {
      setIsLoading(false)
    }
  }

  return {
    isLoading,
    getPkByRound,
  }
}
