import React, { Fragment, useEffect, useState } from 'react'
import CardContent from '@/components/Cards/CardContent'
import VotesBadge from '@/components/VotesBadge'
import PollCardResult from '@/components/Cards/PollCardResult'
import { convertPollData, formatDate, markWinner } from '@/utils/methods'
import PastPollSection from '@/pages/Landing/components/PastPoll'
import { useParams } from 'react-router-dom'
import LoadingAnimation from '@/components/LoadingAnimation'
import { PollResult as PollResultType } from '@/model/poll.model'
import { useVoteManagementContext } from '@/context/voteManagement'

const PollResult: React.FC = () => {
  const params = useParams()
  const { roundId } = params
  const { pastPolls, getWebResult } = useVoteManagementContext()
  const [loading, setLoading] = useState<boolean>(true)
  const [poll, setPoll] = useState<PollResultType | null>(null)

  useEffect(() => {
    if (pastPolls.length && roundId) {
      const currentPoll = pastPolls.find((poll) => poll.roundId === parseInt(roundId))
      if (currentPoll) {
        setPoll(currentPoll)
        setLoading(false)
      }
    }
  }, [pastPolls, roundId])

  useEffect(() => {
    if (!pastPolls.length && roundId) {
      const fetchPoll = async () => {
        const pollResult = await getWebResult(parseInt(roundId))
        if (pollResult) {
          const convertedPoll = convertPollData(pollResult)
          setPoll(convertedPoll)
          setLoading(false)
        }
      }
      fetchPoll()
    }
  }, [pastPolls, roundId])

  return (
    <div className='mb-28 flex min-h-[730px] w-screen flex-col items-center justify-center  space-y-16 px-6'>
      {loading && !poll && (
        <div className='flex items-center justify-center'>
          <LoadingAnimation isLoading={loading} />
        </div>
      )}
      {!loading && poll && (
        <Fragment>
          <div className='my-28 flex w-full flex-col items-center justify-center space-y-12'>
            <div className='flex flex-col items-center justify-center space-y-6'>
              <div className='space-y-2 text-center'>
                <p className='text-sm font-extrabold uppercase'>daily poll</p>
                <h1 className='text-h1 font-bold text-slate-600'>Results</h1>
                <p className=' text-2xl font-bold'>{formatDate(poll.date)}</p>
              </div>

              <VotesBadge totalVotes={poll.totalVotes} />
            </div>
            <PollCardResult results={markWinner(poll.options)} totalVotes={poll.totalVotes} isResult />
          </div>

          <CardContent>
            <div className='space-y-4'>
              <p className='text-base font-extrabold uppercase text-slate-600/50'>WHAT JUST HAPPENED?</p>
              <div className='space-y-2'>
                <p className='text-xl leading-8 text-slate-600'>
                  After casting your vote, CRISP securely processed your selection using a blend of Fully Homomorphic Encryption (FHE),
                  threshold cryptography, and zero-knowledge proofs (ZKPs), without revealing your identity or choice. Your vote was
                  encrypted and anonymously aggregated with others, ensuring the integrity of the voting process while strictly maintaining
                  confidentiality. The protocol's advanced cryptographic techniques guarantee that your vote contributes to the final
                  outcome without any risk of privacy breaches or undue influence.
                </p>
                {/* <div className='flex cursor-pointer items-center space-x-2' onClick={() => setShowCode(!showCode)}>
                  <p className='text-lime-400 underline'>See what&apos;s happening under the hood</p>
                  <img src={CircleIcon} className='h-[18] w-[18]' />
                </div>
                {showCode && <CodeTextDisplay />} */}
              </div>
            </div>
            <div className='space-y-4'>
              <p className='text-base font-extrabold uppercase text-slate-600/50'>WHAT DOES THIS MEAN?</p>
              <p className='text-xl leading-8 text-slate-600'>
                Your participation has directly contributed to a transparent and fair decision-making process, showcasing the power of
                privacy-preserving technology in governance and beyond. The use of CRISP in this vote represents a significant step towards
                secure, anonymous, and tamper-proof digital elections and polls. This innovation ensures that every vote counts equally
                while safeguarding against the risks of fraud and collusion, enhancing the reliability and trustworthiness of digital
                decision-making platforms.
              </p>
            </div>
          </CardContent>
          {pastPolls.length > 0 && <PastPollSection customLabel='Historic polls' />}
        </Fragment>
      )}
    </div>
  )
}

export default PollResult
