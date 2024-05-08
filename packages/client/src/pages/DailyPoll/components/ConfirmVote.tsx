import React from 'react'
import CountdownTimer from '@/components/CountdownTime'
import CardContent from '@/components/Cards/CardContent'
import { Link } from 'react-router-dom'

type ConfirmVoteProps = {
  endTime: Date
}
const ConfirmVote: React.FC<ConfirmVoteProps> = ({ endTime }) => {
  return (
    <div className='my-28 flex w-screen flex-col items-center justify-center space-y-12'>
      <div className='space-y-2 text-center'>
        <p className='text-sm font-extrabold uppercase'>daily poll</p>
        <h1 className='text-h1 font-bold text-slate-600'>Thanks for voting!</h1>
      </div>
      <div className='flex flex-col justify-center space-y-6'>
        <CountdownTimer endTime={endTime} />
        {/* <button className='button-outlined button-max w-[140]'>notify me</button> */}
      </div>
      <CardContent>
        <div className='space-y-4'>
          <p className='text-base font-extrabold uppercase text-slate-600/50'>WHAT JUST HAPPENED?</p>
          <div className='space-y-2'>
            <p className='text-xl leading-8 text-slate-600'>
              After casting your vote, CRISP securely processed your selection using a blend of Fully Homomorphic Encryption (FHE),
              threshold cryptography, and zero-knowledge proofs (ZKPs), without revealing your identity or choice. Your vote was encrypted
              and anonymously aggregated with others, ensuring the integrity of the voting process while strictly maintaining
              confidentiality. The protocol's advanced cryptographic techniques guarantee that your vote contributes to the final outcome
              without any risk of privacy breaches or undue influence.
            </p>
          </div>
        </div>
        <div className='space-y-4'>
          <p className='text-base font-extrabold uppercase text-slate-600/50'>WHAT DOES THIS MEAN?</p>
          <p className='text-xl leading-8 text-slate-600'>
            Your participation has directly contributed to a transparent and fair decision-making process, showcasing the power of
            privacy-preserving technology in governance and beyond. The use of CRISP in this vote represents a significant step towards
            secure, anonymous, and tamper-proof digital elections and polls. This innovation ensures that every vote counts equally while
            safeguarding against the risks of fraud and collusion, enhancing the reliability and trustworthiness of digital decision-making
            platforms.
          </p>
        </div>
      </CardContent>
    </div>
  )
}

export default ConfirmVote
