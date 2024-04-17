import { createGenericContext } from '@/utils/create-generic-context'
import { VoteManagementContextType, VoteManagementProviderProps } from '@/context/voteManagement'
import { useWebAssemblyHook } from '@/hooks/wasm/useWebAssembly'
import { useState } from 'react'
import { SocialAuth } from '@/model/twitter.model'
import useLocalStorage from '@/hooks/generic/useLocalStorage'

const [useVoteManagementContext, VoteManagementContextProvider] = createGenericContext<VoteManagementContextType>()

const VoteManagementProvider = ({ children }: VoteManagementProviderProps) => {
  /**
   * Voting Management States
   **/
  const [socialAuth, setSocialAuth] = useLocalStorage<SocialAuth | null>('socialAuth', null)
  const [user, setUser] = useState<SocialAuth | null>(socialAuth)

  /**
   * Voting Management Methods
   **/
  const { isLoading, wasmInstance, encryptInstance, initWebAssembly, encryptVote } = useWebAssemblyHook()
  const logout = () => {
    setUser(null)
    setSocialAuth(null)
  }

  return (
    <VoteManagementContextProvider
      value={{
        isLoading,
        wasmInstance,
        encryptInstance,
        user,
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
