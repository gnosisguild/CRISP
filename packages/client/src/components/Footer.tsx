import React from 'react'
import GnosisGuildLogo from '@/assets/icons/gg.svg'
import EnclaveLogo from '@/assets/icons/enclaveLogo.svg'
import { Link } from 'react-router-dom'

const Footer: React.FC = () => {
  return (
    <footer className='relative z-10 w-full border-t-2 border-slate-600/20 bg-slate-200 p-6'>
      <div className='mx-auto flex w-full max-w-screen-xl flex-col items-center justify-between gap-4 md:flex-row'>
        <Link to='/whitepaper#' className='duration-300 ease-in-out hover:opacity-70'>
          <p className='text-sm font-bold'>CRISP Whitepaper</p>
        </Link>
        <Link to='https://www.gnosisguild.org/' target='_blank' className='flex flex-col items-center gap-2 md:flex-row'>
          <div className='flex items-center gap-2'>
            <p className='text-sm'>Secured with</p>
            <img src={EnclaveLogo} className='h-6 w-auto cursor-pointer duration-300 ease-in-out hover:opacity-70' />
          </div>
          <div className='flex items-center gap-2'>
            <p className='text-sm'>built by</p>
            <div className='flex items-center gap-2 duration-300 ease-in-out hover:opacity-70'>
              <p className='text-sm font-bold'>Gnosis Guild</p>
              <img src={GnosisGuildLogo} className='h-6 w-6' />
            </div>
          </div>
        </Link>
      </div>
    </footer>
  )
}

export default Footer
