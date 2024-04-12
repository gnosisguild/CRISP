import React, { useState } from 'react'
import Card from '../../../components/Cards/Card'
import { DAILY_POLL } from '../../../mocks/polls'
import { Poll } from '../../../model/poll.model'
import Modal from '../../../components/Modal'
import RegisterModal from '../../Register/Register'

type DailyPollSectionProps = {
  isScreen?: boolean
  onVoted?: () => void
}

const DailyPollSection: React.FC<DailyPollSectionProps> = ({ isScreen, onVoted }) => {
  const [pollOptions, setPollOptions] = useState<Poll[]>(DAILY_POLL)
  const [noPollSelected, setPollSelected] = useState<boolean>(true)
  const [modalOpen, setModalOpen] = useState(false)

  const openModal = () => setModalOpen(true)
  const closeModal = () => setModalOpen(false)

  const handleChecked = (selectedId: number) => {
    const updatedOptions = pollOptions.map((option) => ({
      ...option,
      checked: !option.checked && option.id === selectedId,
    }))

    setPollOptions(updatedOptions)
    setPollSelected(updatedOptions.every((poll) => !poll.checked))
  }
  return (
    <>
      <div
        className={`flex w-screen flex-col items-center justify-center px-6 ${!isScreen ? 'h-screen' : 'h-screen-minus-header-footer'} `}
      >
        <div className='mx-auto flex w-full max-w-screen-md flex-col items-center justify-center space-y-12'>
          <div className='space-y-2'>
            <p className='text-center text-sm font-extrabold uppercase text-zinc-900'>Daily Poll</p>
            <h3 className='font-bold leading-none text-twilight-blue-900'>Choose your favorite</h3>
          </div>
          <div className='flex items-center justify-center space-x-2'>
            <div className='flex items-center space-x-2 rounded-lg border-2 border-green-dark-800 bg-green-light px-2 py-1 text-center font-bold uppercase leading-none text-white-900'>
              <div className='h-1.5 w-1.5 animate-pulse rounded-full bg-white-900'></div>
              <div>Live</div>
            </div>
            <div className='rounded-lg border-2 border-twilight-blue-200 bg-white-900 px-2 py-1.5 text-center font-bold uppercase leading-none text-zinc-500'>
              23 votes
            </div>
          </div>
          <div className='grid w-full grid-cols-2 gap-4 md:gap-8'>
            {pollOptions.map((poll) => (
              <div key={poll.id} className='col-span-2 md:col-span-1'>
                <Card checked={poll.checked} onChecked={() => handleChecked(poll.id)}>
                  <p className='inline-block text-8xl leading-none'>{poll.label}</p>
                </Card>
              </div>
            ))}
          </div>
          <button
            className={`button-outlined button-max ${noPollSelected ? 'button-disabled' : ''}`}
            disabled={noPollSelected}
            onClick={openModal}
          >
            cast vote
          </button>
        </div>
      </div>
      <Modal show={modalOpen} onClose={closeModal}>
        <RegisterModal onClose={closeModal} />
      </Modal>
    </>
  )
}

export default DailyPollSection
