import { createGenericContext } from '@/utils/create-generic-context'
import { VoteManagementContextType, VoteManagementProviderProps } from '@/context/voteManagement'
import { useWebAssemblyHook } from '@/hooks/wasm/useWebAssembly'

const [useVoteManagementContext, VoteManagementContextProvider] = createGenericContext<VoteManagementContextType>()

const VoteManagementProvider = ({ children }: VoteManagementProviderProps) => {
  /**
   * Voting Management States
   **/

  /**
   * Voting Management Methods
   **/
  const { isLoading, wasmInstance, encryptInstance, initWebAssembly, encryptVote } = useWebAssemblyHook()

  return (
    <VoteManagementContextProvider
      value={{
        isLoading,
        wasmInstance,
        encryptInstance,
        initWebAssembly,
        encryptVote,
      }}
    >
      {children}
    </VoteManagementContextProvider>
  )
}

export { useVoteManagementContext, VoteManagementProvider }
