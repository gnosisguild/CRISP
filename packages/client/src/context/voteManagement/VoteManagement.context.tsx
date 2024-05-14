import { createGenericContext } from '@/utils/create-generic-context'
import { VoteManagementContextType, VoteManagementProviderProps } from '@/context/voteManagement'
import { useWebAssemblyHook } from '@/hooks/wasm/useWebAssembly'
import { useEffect, useState } from 'react'
import { SocialAuth } from '@/model/twitter.model'
import useLocalStorage from '@/hooks/generic/useLocalStorage'
import { VoteStateLite, VotingRound } from '@/model/vote.model'
import { useEnclaveServer } from '@/hooks/enclave/useEnclaveServer'
import { convertPollData, convertTimestampToDate } from '@/utils/methods'
import { Poll, PollResult } from '@/model/poll.model'
import { generatePoll } from '@/utils/generate-random-poll'
import { handleGenericError } from '@/utils/handle-generic-error'
import useCircuitHook from '@/hooks/wasm/useCircuit'

const [useVoteManagementContext, VoteManagementContextProvider] = createGenericContext<VoteManagementContextType>()

const VoteManagementProvider = ({ children }: VoteManagementProviderProps) => {
  /**
   * Voting Management States
   **/
  const [socialAuth, setSocialAuth] = useLocalStorage<SocialAuth | null>('socialAuth', null)
  const [user, setUser] = useState<SocialAuth | null>(socialAuth)
  const [roundState, setRoundState] = useState<VoteStateLite | null>(null)
  const [votingRound, setVotingRound] = useState<VotingRound | null>(null)
  const [roundEndDate, setRoundEndDate] = useState<Date | null>(null)
  const [isLoading, setIsLoading] = useState<boolean>(false)
  const [pollOptions, setPollOptions] = useState<Poll[]>([])
  const [pastPolls, setPastPolls] = useState<PollResult[]>([])

  /**
   * Voting Management Methods
   **/
  const { isLoading: wasmLoading, wasmInstance, encryptInstance, initWebAssembly, encryptVote } = useWebAssemblyHook()
  const { proveVote, initCircuits, zpkClient } = useCircuitHook()
  const {
    isLoading: enclaveLoading,
    getRoundStateLite: getRoundStateLiteRequest,
    getWebResult,
    getRound,
    broadcastVote,
  } = useEnclaveServer()

  const initialLoad = async () => {
    await initWebAssembly()
    await initCircuits()
    const round = await getRound()
    if (round) {
      await getRoundStateLite(round.round_count)
    }
  }

  const existNewRound = async () => {
    const round = await getRound()
    if (round && votingRound && round.round_count > votingRound.round_id) {
      await getRoundStateLite(round.round_count)
    }
  }

  const logout = () => {
    setUser(null)
    setSocialAuth(null)
  }

  const getRoundStateLite = async (roundCount: number) => {
    const roundState = await getRoundStateLiteRequest(roundCount)
    if (roundState) {
      setRoundState(roundState)
      setVotingRound({ round_id: roundState.id, pk_bytes: roundState.pk })
      setPollOptions(generatePoll({ round_id: roundState.id, emojis: roundState.emojis }))
      setRoundEndDate(convertTimestampToDate(roundState.start_time, roundState.poll_length))
    }
  }

  const getPastPolls = async (roundCount: number) => {
    let results: PollResult[] = []
    try {
      for (let i = 0; i < roundCount; i++) {
        const result = await getWebResult(i + 1)
        if (result) {
          const convertedPoll = convertPollData(result)
          results.push(convertedPoll)
        }
        await new Promise((resolve) => setTimeout(resolve, 500))
      }
      setPastPolls(results)
    } catch (error) {
      handleGenericError('getPastPolls', error as Error)
    }
  }

  useEffect(() => {
    if ([wasmLoading, enclaveLoading].includes(true)) {
      return setIsLoading(true)
    }
    setIsLoading(false)
  }, [wasmLoading, enclaveLoading])

  return (
    <VoteManagementContextProvider
      value={{
        isLoading,
        wasmInstance,
        encryptInstance,
        user,
        votingRound,
        roundEndDate,
        pollOptions,
        roundState,
        pastPolls,
        zpkClient,
        proveVote,
        existNewRound,
        getWebResult,
        setPastPolls,
        getPastPolls,
        getRoundStateLite,
        setPollOptions,
        initialLoad,
        broadcastVote,
        setVotingRound,
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
