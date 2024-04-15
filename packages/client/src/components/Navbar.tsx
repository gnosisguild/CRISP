import React from 'react'
import Logo from '../assets/icons/logo.svg'
import { useNavigate } from 'react-router-dom'
import NavMenu from './NavMenu'

const PAGES = [
  {
    label: 'About',
    path: '/about',
  },
  // {
  //   label: 'Daily Polls',
  //   path: '/daily',
  // },
  {
    label: 'Docs',
    path: '/docs',
  },
  // {
  //   label: 'Historic Polls',
  //   path: '/historic',
  // },
]

const Navbar: React.FC = () => {
  const navigate = useNavigate()

  const handleNavigation = (path: string) => {
    navigate(path)
  }

  return (
    <nav className='border-b-2 border-twilight-blue-200 bg-white-900 shadow-md '>
      <div className='mx-auto max-w-7xl px-4 sm:px-2 lg:px-9'>
        <div className='flex h-20 items-center justify-between'>
          <img src={Logo} alt='CRISP Logo' className='h-8 cursor-pointer' onClick={() => navigate('/')} />
          <div className='hidden sm:flex sm:items-center sm:gap-8'>
            {PAGES.map(({ label, path }) => (
              <a
                key={label}
                onClick={() => handleNavigation(path)}
                className='hover:text-twilight-blue-600 cursor-pointer font-bold text-twilight-blue-900'
              >
                {label}
              </a>
            ))}
            <NavMenu />
          </div>
        </div>
      </div>
    </nav>
  )
}

export default Navbar
