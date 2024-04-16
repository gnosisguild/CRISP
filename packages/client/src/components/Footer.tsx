import React from 'react'
import GnosisGuildLogo from '../assets/icons/gg.svg'
import EnclaveLogo from '../assets/icons/enclaveLogo.svg'
import { Link } from 'react-router-dom'

const Footer: React.FC = () => {
  return (
    <footer className='relative z-10 w-screen border-t-2 border-slate-600/20 bg-slate-200 p-6'>
      <div className='mx-auto w-full max-w-screen-xl'>
        <div className='flex items-center justify-between gap-2'>
          <Link to='/whitepaper#' className='duration-300 ease-in-out hover:opacity-70'>
            <p className='text-sm font-bold'>CRISP Whitepaper</p>
          </Link>
          <div className='flex items-center space-x-2'>
            <p className='flex items-center space-x-2 text-sm'>Secured with </p>
            <img src={EnclaveLogo} className='h-[24] w-auto cursor-pointer duration-300 ease-in-out hover:opacity-70' />
            <div className='flex items-center space-x-2'>
              <p className='text-sm'>built by</p>
              <Link
                to='https://www.gnosisguild.org/'
                target='_blank'
                className='flex items-center space-x-2 duration-300 ease-in-out hover:opacity-70'
              >
                <p className='text-sm font-bold'> Gnosis Guild</p>
                <img src={GnosisGuildLogo} className='h-[24] w-[24]' />
              </Link>
            </div>
          </div>
        </div>
      </div>
    </footer>
  )
}

export default Footer
