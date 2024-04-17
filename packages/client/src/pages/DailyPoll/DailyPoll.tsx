import React, { Fragment, useState } from 'react'
import DailyPollSection from '@/pages/Landing/components/DailyPoll'
import ConfirmVote from '@/pages/DailyPoll/components/ConfirmVote'
import { Poll } from '@/model/poll.model'
import { handleGenericError } from '@/utils/handle-generic-error'
import { Contract, Interface, JsonRpcProvider, Wallet } from 'ethers'
import { RFV_ABI } from '@/contracts/rfv/abi/rfvAbi'
import { RFV_CONTRACT_ADDRESS } from '@/contracts/rfv'
import { useVoteManagementContext } from '@/context/voteManagement'

const INFURA_KEY = import.meta.env.VITE_INFURA_API_KEY
const PRIVATE_KEY = import.meta.env.VITE_PRIVATE_KEY

if (!INFURA_KEY) handleGenericError('useTwitter', { name: 'INFURA_KEY', message: 'Missing env VITE_INFURA_API_KEY' })
if (!PRIVATE_KEY) handleGenericError('useTwitter', { name: 'PRIVATE_KEY', message: 'Missing env VITE_PRIVATE_KEY' })

const DailyPoll: React.FC = () => {
  const { encryptVote, votingRound } = useVoteManagementContext()
  const [endTime, setEndTime] = useState<Date | null>(null)
  const [voteCompleted, setVotedCompleted] = useState<boolean>(false)
  const [loading, setLoading] = useState<boolean>(false)

  const handleVoted = async (vote: Poll | null) => {
    if (vote && votingRound) {
      setLoading(true)
      const provider = new JsonRpcProvider(`https://sepolia.infura.io/v3/${INFURA_KEY}`)
      const contractInterface = new Interface(RFV_ABI)
      const wallet = new Wallet(PRIVATE_KEY, provider)
      const contract = new Contract(RFV_CONTRACT_ADDRESS, contractInterface, wallet)
      if (!provider) {
        handleGenericError('handleVoted', { name: 'Provider', message: 'Error: provider' })
        return
      }
      const voteEncrypted = await encryptVote(BigInt(1), new Uint8Array(votingRound.pk_bytes))
      try {
        const tx = await contract.voteEncrypted(voteEncrypted)
        console.log('tx', tx)
        const newDate = new Date()
        newDate.setHours(newDate.getHours() + 28)
        setEndTime(newDate)
        setVotedCompleted(true)
      } catch (error) {
        handleGenericError('Vote Encrypted Tx', error as Error)
      } finally {
        setLoading(false)
      }
    }
  }

  return (
    <Fragment>
      {!voteCompleted && <DailyPollSection onVoted={handleVoted} loading={loading} />}
      {voteCompleted && endTime && <ConfirmVote endTime={endTime} />}
    </Fragment>
  )
}

export default DailyPoll
