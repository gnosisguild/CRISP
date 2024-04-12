import React, { useState } from 'react'
import Logo from '../../../assets/icons/logo.svg'
import CircularTiles from '../../../components/CircularTiles'
import { ArrowSquareOut, Keyhole, ListMagnifyingGlass, ShieldCheck } from '@phosphor-icons/react'
import DailyPollSection from './DailyPoll'

const HeroSection: React.FC = () => {
  const [showDailyPoll, setShowDailyPoll] = useState(false)

  return (
    <>
      {showDailyPoll ? (
        <DailyPollSection />
      ) : (
        <div className='relative flex min-h-screen w-screen items-center justify-center px-6'>
          <div className='absolute bottom-1 right-0 w-[40vh] space-y-2'>
            <CircularTiles count={2} />
          </div>
          <div className='relative mx-auto w-full max-w-screen-md space-y-12'>
            <div className='space-y-4'>
              <h3 className='text-3xl font-normal leading-none text-zinc-400'>Introducing</h3>
              <img src={Logo} alt='CRISP Logo' className='h-20' />
              <h4 className='w-full text-base leading-none text-zinc-500'>Collusion-Resistant Impartial Selection Protocol</h4>
            </div>
            <ul className='space-y-3'>
              <li className='flex items-center space-x-2'>
                <Keyhole className='text-green-dark-800' size={32} />
                <div className='text-lg text-zinc-400'>
                  <span className='mr-1 font-bold text-green-dark-800'>Private.</span>
                  Voter privacy through advaned encryption.
                </div>
              </li>
              <li className='flex items-center space-x-2'>
                <ListMagnifyingGlass className='text-green-dark-800' size={32} />
                <div className='text-lg text-zinc-400'>
                  <span className='mr-1 font-bold text-green-dark-800'>Reliable.</span>
                  Verifiable results while preserving confidentiality.
                </div>
              </li>
              <li className='flex items-center space-x-2'>
                <ShieldCheck className='text-green-dark-800' size={32} />
                <div className='text-lg text-zinc-400'>
                  <span className='mr-1 font-bold text-green-dark-800'>Equitable.</span>
                  Robust safeguards against collusion and tampering.
                </div>
              </li>
            </ul>
            <div className='space-y-3'>
              <button className='button button-max' onClick={() => setShowDailyPoll(true)}>
                Try Demo
              </button>
              <a
                href='/about'
                className='inline-flex cursor-pointer items-center space-x-2 text-green-dark-900 duration-300 ease-in-out hover:opacity-70'
              >
                <ArrowSquareOut size={20} weight='bold' />
                <div>Learn more.</div>
              </a>
            </div>
          </div>
        </div>
      )}
    </>
  )
}

export default HeroSection
