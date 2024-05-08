import React from 'react'
import Logo from '@/assets/icons/logo.svg'
import { Link } from 'react-router-dom'
import NavMenu from '@/components/NavMenu'

const PAGES = [
  {
    label: 'About',
    path: '/about',
  },
]

const Navbar: React.FC = () => {
  return (
    <nav className='absolute left-0 top-0 z-10 w-screen px-6 lg:px-9'>
      <div className='mx-auto max-w-screen-xl'>
        <div className='flex h-20 items-center justify-between'>
          <Link
            to={'/'}
            className='hover:text-twilight-blue-600 cursor-pointer font-bold text-slate-600 duration-300 ease-in-out hover:opacity-70'
          >
            <img src={Logo} alt='CRISP Logo' className='h-6 md:h-8 cursor-pointer duration-300 ease-in-out hover:opacity-70' />
          </Link>

          <div className='flex items-center gap-8'>
            {PAGES.map(({ label, path }) => (
              <Link
              key={label}
              to={path}
              className='max-md:hidden hover:text-twilight-blue-600 cursor-pointer font-bold text-slate-600 duration-300 ease-in-out hover:opacity-70'
              >
                {label}
              </Link>
            ))}
            <NavMenu />
          </div>
        </div>
      </div>
    </nav>
  )
}

export default Navbar
