import React, { useState } from 'react'
import Card from '@/components/Cards/Card'
import { DAILY_POLL } from '@/mocks/polls'
import { Poll } from '@/model/poll.model'
import Modal from '@/components/Modal'
import RegisterModal from '@/pages/Register/Register'

type DailyPollSectionProps = {
  isScreen?: boolean
  onVoted?: () => void
}

const DailyPollSection: React.FC<DailyPollSectionProps> = ({ onVoted }) => {
  const [pollOptions, setPollOptions] = useState<Poll[]>(DAILY_POLL)
  const [noPollSelected, setPollSelected] = useState<boolean>(true)
  const [modalOpen, setModalOpen] = useState(false)

  const openModal = () => setModalOpen(true)
  const closeModal = () => {
    onVoted && onVoted()
    setModalOpen(false)
  }

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
      <div className='flex h-screen w-screen flex-col items-center justify-center px-6'>
        <div className='mx-auto flex w-full max-w-screen-md flex-col items-center justify-center space-y-12'>
          <div className='space-y-2'>
            <p className='text-center text-sm font-extrabold uppercase text-slate-800'>Daily Poll</p>
            <h3 className='font-bold leading-none text-slate-600'>Choose your favorite</h3>
          </div>
          <div className='flex items-center justify-center space-x-2'>
            <div className='flex items-center space-x-2 rounded-lg border-2 border-lime-600/80 bg-lime-400 px-2 py-1 text-center font-bold uppercase leading-none text-white'>
              <div className='h-1.5 w-1.5 animate-pulse rounded-full bg-white'></div>
              <div>Live</div>
            </div>
            <div className='rounded-lg border-2 border-slate-600/20 bg-white px-2 py-1.5 text-center font-bold uppercase leading-none text-slate-800/50'>
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
          <div className='space-y-4'>
            {noPollSelected && <div className='text-center text-sm leading-none text-slate-500'>Select your favorite</div>}
            <button
              className={`button-outlined button-max ${noPollSelected ? 'button-disabled' : ''}`}
              disabled={noPollSelected}
              onClick={openModal}
            >
              cast vote
            </button>
          </div>
        </div>
      </div>
      <Modal show={modalOpen} onClose={closeModal}>
        <RegisterModal onClose={closeModal} />
      </Modal>
    </>
  )
}

export default DailyPollSection
