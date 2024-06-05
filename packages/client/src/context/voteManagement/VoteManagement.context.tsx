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
  const [txUrl, setTxUrl] = useState<string | undefined>(undefined)
  const [pollResult, setPollResult] = useState<PollResult | null>(null)

  /**
   * Voting Management Methods
   **/
  const { isLoading: wasmLoading, wasmInstance, encryptInstance, initWebAssembly, encryptVote } = useWebAssemblyHook()
  const {
    isLoading: enclaveLoading,
    getRoundStateLite: getRoundStateLiteRequest,
    getWebResultByRound,
    getToken,
    getWebResult,
    getRound,
    broadcastVote,
  } = useEnclaveServer()

  const initialLoad = async () => {
    await initWebAssembly()
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

    if (roundState?.pk.length === 1 && roundState.pk[0] === 0) {
      handleGenericError('getRoundStateLite', {
        message: 'Enclave server failed generating the necessary pk bytes',
        name: 'getRoundStateLite',
      })
    }
    if (roundState) {
      setRoundState(roundState)
      setVotingRound({ round_id: roundState.id, pk_bytes: roundState.pk })
      setPollOptions(generatePoll({ round_id: roundState.id, emojis: roundState.emojis }))
      setRoundEndDate(convertTimestampToDate(roundState.start_time, roundState.poll_length))
    }
  }

  const getPastPolls = async () => {
    try {
      const result = await getWebResult()
      if (result) {
        const convertedPolls = convertPollData(result)
        setPastPolls(convertedPolls)
      }
    } catch (error) {
      handleGenericError('getPastPolls', error as Error)
    } finally {
      setIsLoading(false)
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
        txUrl,
        pollResult,
        setPollResult,
        getWebResultByRound,
        getToken,
        setTxUrl,
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
