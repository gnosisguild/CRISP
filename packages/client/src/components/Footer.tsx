import React from 'react'
import { useLocation } from 'react-router-dom'
import GnosisGuildLogo from '../assets/icons/gg.svg'

const Footer: React.FC = () => {
  const location = useLocation()
  const path = location.pathname

  const goToGnosisGuild = () => window.open('https://www.gnosisguild.org/', '_blank')

  return (
    <footer
      className={`${path === '/' ? 'footer-dynamic' : 'footer-fixed'} mx-auto flex h-16 w-full items-center justify-center bg-mist-900 py-6`}
    >
      <div className='flex w-full max-w-[900px] items-center justify-between border-t-2 border-twilight-blue-200 py-6'>
        <p className='text-sm font-bold'>CRISP Whitepaper</p>
        <div className='flex cursor-pointer items-center space-x-2' onClick={goToGnosisGuild}>
          <p className='text-sm'>
            Built by <span className='font-bold'> Gnosis Guild</span>
          </p>
          <img src={GnosisGuildLogo} className='h-[24] w-[24]' />
        </div>
      </div>
    </footer>
  )
}

export default Footer
