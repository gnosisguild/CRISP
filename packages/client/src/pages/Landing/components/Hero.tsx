import React, { useState } from 'react'
import Hero from '../../../assets/images/hero.svg'
import Logo from '../../../assets/icons/logo.svg'
import Modal from '../../../components/Modal'
import RegisterModal from '../../Register/Register'

const HeroSection: React.FC = () => {
  const [modalOpen, setModalOpen] = useState(false)

  const openModal = () => setModalOpen(true)
  const closeModal = () => setModalOpen(false)

  return (
    <div
      className='flex min-h-screen w-screen items-center justify-center px-6'
      style={{
        backgroundImage: `url(${Hero})`,
        backgroundSize: 'cover',
        backgroundPosition: `center`,
      }}
    >
      <div className='mx-auto w-full max-w-screen-md space-y-12'>
        <div className='flex w-full flex-col items-center justify-center space-y-5'>
          <h3 className='text-center text-zinc-900'>Introducing</h3>
          <img src={Logo} alt='CRISP Logo' className='h-20' />
          <h4 className='w-full text-center text-gray-900'>Collusion-Resistant Impartial Selection Protocol</h4>
        </div>
        <div className='flex w-full items-center justify-center space-x-6'>
          <button className='button button-max' onClick={openModal}>
            Register
          </button>
          <button className='button-outlined button-max'>Learn More</button>
        </div>
      </div>
      <Modal show={modalOpen} onClose={closeModal}>
        <RegisterModal onClose={closeModal} />
      </Modal>
    </div>
  )
}

export default HeroSection
