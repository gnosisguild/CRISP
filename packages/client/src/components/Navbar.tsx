import React, { useState } from 'react'
import Logo from '../assets/icons/logo.svg'
import Modal from './Modal'
import RegisterModal from '../pages/Register/Register'

const Navbar: React.FC = () => {
  const [modalOpen, setModalOpen] = useState(false)

  const openModal = () => setModalOpen(true)
  const closeModal = () => setModalOpen(false)
  return (
    <nav className='border-b-2 border-twilight-blue-200 bg-white-900 shadow-md '>
      <div className='mx-auto max-w-7xl px-4 sm:px-2 lg:px-9'>
        <div className='flex h-20 items-center justify-between'>
          {/* Logo */}
          <img src={Logo} alt='CRISP Logo' className='h-8' />

          {/* Links de navegación */}
          <div className='hidden sm:flex sm:items-center sm:gap-8'>
            <a href='#about' className='hover:text-twilight-blue-600 font-bold text-twilight-blue-900'>
              About
            </a>
            <a href='#daily-polls' className='hover:text-twilight-blue-600 font-bold text-twilight-blue-900'>
              Daily Polls
            </a>
            <a href='#historic-polls' className='hover:text-twilight-blue-600 font-bold text-twilight-blue-900'>
              Historic Polls
            </a>
          </div>

          {/* Botones de acción */}
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
