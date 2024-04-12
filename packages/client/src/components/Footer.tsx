import React from 'react'
import { useLocation } from 'react-router-dom'
import GnosisGuildLogo from '../assets/icons/gg.svg'

const Footer: React.FC = () => {
  const location = useLocation()
  const path = location.pathname

  const goToGnosisGuild = () => window.open('https://www.gnosisguild.org/', '_blank')

  return (
    <footer className='w-screen border-t-2 border-slate-600/20 bg-slate-200 p-6'>
      <div className='mx-auto w-full max-w-screen-xl'>
        <div className='flex items-center justify-between gap-2'>
          <p className='text-sm font-bold'>CRISP Whitepaper</p>
          <div className='flex cursor-pointer items-center space-x-2' onClick={goToGnosisGuild}>
            <p className='text-sm'>
              Built by <span className='font-bold'> Gnosis Guild</span>
            </p>
            <img src={GnosisGuildLogo} className='h-[24] w-[24]' />
          </div>
        </div>
      </div>
    </footer>
  )
}

export default Footer
