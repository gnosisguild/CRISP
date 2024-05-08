import React from 'react'
// import CircleIcon from '@/assets/icons/caretCircle.svg'
import CardContent from '@/components/Cards/CardContent'
import CircularTiles from '@/components/CircularTiles'
import { Link } from 'react-router-dom'

const About: React.FC = () => {
  return (
    <div className='relative flex w-full flex-1 items-center justify-center px-6 py-28'>
      <div className='absolute bottom-0 right-0 grid w-full grid-cols-2 gap-2 max-md:opacity-50 md:w-[70vh]'>
        <CircularTiles count={4} />
      </div>
      <div className='relative space-y-12'>
        <h1 className='text-h1 font-bold text-slate-600'>About CRISP</h1>
        <CardContent>
          <div className='space-y-4'>
            <p className='text-base font-extrabold uppercase text-slate-600/50'>what is crisp?</p>
            <div className='space-y-2'>
              <p className='leading-8 text-slate-600'>
                CRISP (Collusion-Resistant Impartial Selection Protocol) is secure protocol for digital decision making, leveraging
                Fully Homomorphic Encryption (FHE) and threshold cryptography to enable verifiable secret ballots; a critical
                component for democracies and many other decision-making applications.
              </p>
              {/* <div className='flex cursor-pointer items-center space-x-2'>
                <p className='text-lime-400 underline'>See what&apos;s happening under the hood</p>
                <img src={CircleIcon} className='h-[18] w-[18]' />
              </div> */}
            </div>
          </div>
          <div className='space-y-4'>
            <p className='text-base font-extrabold uppercase text-slate-600/50'>why is this important?</p>
            <p className='leading-8 text-slate-600'>
              Open ballots are well known to produce sub-optimal outcomes due to bribery and other forms of collusion.
              CRISP mitigates collusion by ensuring ballots are secret and receipt-free, enabling a secure and impartial
              decision-making environemnt.
            </p>
            {/* <div className='flex cursor-pointer items-center space-x-2'>
              <p className='text-lime-400 underline'>See what&apos;s happening under the hood</p>
              <img src={CircleIcon} className='h-[18] w-[18] ' />
            </div> */}
          </div>
          <div className='space-y-4'>
            <p className='text-base font-extrabold uppercase text-slate-600/50'>Proof of Concept</p>
            <p className='leading-8 text-slate-600'>
              This application is a Proof of Concept (PoC), demonstrating the viability of Enclave as a network and CRISP
              as an application for secret ballots. For the sake getting a demonstration of CRISP into the wild, this PoC
              application is not yet leveraging Enclave and ommits several key components of CRISP. Future iterations of
              this and other applications will be progressively more complete.
            </p>
            {/* <div className='flex cursor-pointer items-center space-x-2'>
              <p className='text-lime-400 underline'>See what&apos;s happening under the hood</p>
              <img src={CircleIcon} className='h-[18] w-[18] ' />
            </div> */}
          </div>
        </CardContent>
        <Link to='/whitepaper' className='inline-flex'>
          <button className='button-outlined button-max'>view whitepaper</button>
        </Link>
      </div>
    </div>
  )
}

export default About
