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
      style={{
        backgroundImage: `url(${Hero})`,
        backgroundSize: 'cover',
        height: 'calc(100vh - 80px)',
      }}
    >
      <div className='flex h-full items-center justify-center '>
        <div className='flex max-w-[513px] flex-col items-center justify-center'>
          <div className='flex flex-col items-center justify-center space-y-5'>
            <h1 className='text-h1 text-zinc-900'>Introducing</h1>
            <img src={Logo} alt='CRISP Logo' className='h-20' />
            <h2 className='text-center text-h2 text-gray-900'>Collusion-Resistant Impartial Selection Protocol</h2>
          </div>
          <div className='mt-12 flex space-x-6'>
            <button className='button button-max' onClick={openModal}>
              Register
            </button>
            <button className='button-outlined button-max'>Learn More</button>
          </div>
        </div>
      </div>
      <Modal show={modalOpen} onClose={closeModal}>
        <RegisterModal onClose={closeModal} />
      </Modal>
    </div>
  )
}

export default HeroSection
