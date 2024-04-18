import { createGenericContext } from '@/utils/create-generic-context'
import { VoteManagementContextType, VoteManagementProviderProps } from '@/context/voteManagement'
import { useWebAssemblyHook } from '@/hooks/wasm/useWebAssembly'
import { useEffect, useState } from 'react'
import { SocialAuth } from '@/model/twitter.model'
import useLocalStorage from '@/hooks/generic/useLocalStorage'
import { VotingRound } from '@/model/vote.model'
import { useEnclaveServer } from '@/hooks/enclave/useEnclaveServer'

const [useVoteManagementContext, VoteManagementContextProvider] = createGenericContext<VoteManagementContextType>()

const VoteManagementProvider = ({ children }: VoteManagementProviderProps) => {
  /**
   * Voting Management States
   **/
  const [socialAuth, setSocialAuth] = useLocalStorage<SocialAuth | null>('socialAuth', null)
  const [user, setUser] = useState<SocialAuth | null>(socialAuth)
  const [votingRound, setVotingRound] = useState<VotingRound | null>(null)
  const [isLoading, setIsLoading] = useState<boolean>(false)

  /**
   * Voting Management Methods
   **/
  const { isLoading: wasmLoading, wasmInstance, encryptInstance, initWebAssembly, encryptVote } = useWebAssemblyHook()
  const { isLoading: enclaveLoading, getPkByRound: getPkByRoundRequest } = useEnclaveServer()

  const logout = () => {
    setUser(null)
    setSocialAuth(null)
  }
  const getPkByRound = async (votingRound: VotingRound) => {
    const round = await getPkByRoundRequest(votingRound)
    setVotingRound(round ?? null)
    return round
  }

  useEffect(() => {
    if ([wasmLoading, enclaveLoading].includes(true)) {
      return setIsLoading(true)
    }
    setIsLoading(false)
  }, [wasmLoading, enclaveLoading])

  console.log('votingRound', votingRound)

  return (
    <VoteManagementContextProvider
      value={{
        isLoading,
        wasmInstance,
        encryptInstance,
        user,
        votingRound,
        setVotingRound,
        getPkByRound,
        setUser,
        initWebAssembly,
        encryptVote,
        logout,
      }}
    >
      {children}
    </VoteManagementContextProvider>
  )
}

export { useVoteManagementContext, VoteManagementProvider }
