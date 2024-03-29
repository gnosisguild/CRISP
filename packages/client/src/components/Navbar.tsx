import React, { useState } from 'react'
import Logo from '../assets/icons/logo.svg'
import Modal from './Modal'
import RegisterModal from '../pages/Register/Register'
import { useNavigate } from 'react-router-dom'

const PAGES = [
  {
    label: 'About',
    path: '/about',
  },
  {
    label: 'Daily Polls',
    path: '/daily',
  },
  {
    label: 'Historic Polls',
    path: '/historic',
  },
]

const Navbar: React.FC = () => {
  const navigate = useNavigate()
  const [modalOpen, setModalOpen] = useState(false)

  const openModal = () => setModalOpen(true)
  const closeModal = () => setModalOpen(false)

  const handleNavigation = (path: string) => {
    navigate(path)
  }

  return (
    <nav className='border-b-2 border-twilight-blue-200 bg-white-900 shadow-md '>
      <div className='mx-auto max-w-7xl px-4 sm:px-2 lg:px-9'>
        <div className='flex h-20 items-center justify-between'>
          {/* Logo */}
          <img src={Logo} alt='CRISP Logo' className='h-8 cursor-pointer' onClick={() => navigate('/')} />

          <div className='hidden sm:flex sm:items-center sm:gap-8'>
            {PAGES.map(({ label, path }) => (
              <a
                onClick={() => handleNavigation(path)}
                className='hover:text-twilight-blue-600 cursor-pointer font-bold text-twilight-blue-900'
              >
                {label}
              </a>
            ))}
          </div>

          {/* Actions */}
          <div className='flex items-center gap-4'>
            <a href='#login' className='hover:text-twilight-blue-600 font-bold text-twilight-blue-900'>
              Login
            </a>
            <button className='button' onClick={openModal}>
              Register
            </button>
          </div>
          <Modal show={modalOpen} onClose={closeModal}>
            <RegisterModal onClose={closeModal} />
          </Modal>
        </div>
      </div>
    </nav>
  )
}

export default Navbar
