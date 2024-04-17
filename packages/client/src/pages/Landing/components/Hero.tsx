import React from 'react'
import Logo from '@/assets/icons/logo.svg'
import CircularTiles from '@/components/CircularTiles'
import { Link } from 'react-router-dom'
import { Keyhole, ListMagnifyingGlass, ShieldCheck } from '@phosphor-icons/react'

const HeroSection: React.FC = () => {
  return (
    <div className='relative flex min-h-screen w-screen items-center justify-center px-6'>
      <div className='absolute bottom-0 right-0 grid w-[70vh] grid-cols-2 gap-2'>
        <CircularTiles count={4} />
      </div>
      <div className='relative mx-auto w-full max-w-screen-md space-y-12'>
        <div className='space-y-4'>
          <h3 className='text-3xl font-normal leading-none text-zinc-400'>Introducing</h3>
          <img src={Logo} alt='CRISP Logo' className='h-20' />
          <h4 className='w-full text-base leading-none text-slate-800/50'>Collusion-Resistant Impartial Selection Protocol</h4>
        </div>
        <ul className='space-y-3'>
          <li className='flex items-center space-x-2'>
            <Keyhole className='text-lime-600/80' size={32} />
            <div className='text-lg text-zinc-400'>
              <span className='mr-1 font-bold text-lime-600/80'>Private.</span>
              Voter privacy through advanced encryption.
            </div>
          </li>
          <li className='flex items-center space-x-2'>
            <ListMagnifyingGlass className='text-lime-600/80' size={32} />
            <div className='text-lg text-zinc-400'>
              <span className='mr-1 font-bold text-lime-600/80'>Reliable.</span>
              Verifiable results while preserving confidentiality.
            </div>
          </li>
          <li className='flex items-center space-x-2'>
            <ShieldCheck className='text-lime-600/80' size={32} />
            <div className='text-lg text-zinc-400'>
              <span className='mr-1 font-bold text-lime-600/80'>Equitable.</span>
              Robust safeguards against collusion and tampering.
            </div>
          </li>
        </ul>
        <div className='space-y-4'>
          <div className='flex flex-wrap items-center space-x-2 text-sm'>
            <div className='text-slate-400'>This is a simple demonstration of CRISP technology.</div>
            <Link
              to='/about'
              className='inline-flex cursor-pointer items-center space-x-1 text-lime-600 duration-300 ease-in-out hover:underline hover:opacity-70'
            >
              <div>Learn more.</div>
            </Link>
          </div>
          <Link to='/daily' className='inline-flex'>
            <button className='button-primary'>Try Demo</button>
          </Link>
        </div>
      </div>
    </div>
  )
}

export default HeroSection
