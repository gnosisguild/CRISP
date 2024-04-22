import React, { useState } from 'react'
import { Poll } from '@/model/poll.model'
import Card from '@/components/Cards/Card'
import Modal from '@/components/Modal'
import CircularTiles from '@/components/CircularTiles'
import RegisterModal from '@/pages/Register/Register'
import { useVoteManagementContext } from '@/context/voteManagement'
import LoadingAnimation from '@/components/LoadingAnimation'
import { generateRandomPoll } from '@/utils/generate-random-poll'

type DailyPollSectionProps = {
  onVoted?: (vote: Poll) => void
  loading?: boolean
}

const DailyPollSection: React.FC<DailyPollSectionProps> = ({ onVoted, loading }) => {
  const { user } = useVoteManagementContext()
  const [pollOptions, setPollOptions] = useState<Poll[]>(generateRandomPoll())
  const [pollSelected, setPollSelected] = useState<Poll | null>(null)
  const [noPollSelected, setNoPollSelected] = useState<boolean>(true)
  const [modalOpen, setModalOpen] = useState(false)

  const openModal = () => setModalOpen(true)
  const closeModal = () => {
    setModalOpen(false)
  }

  const handleChecked = (selectedId: number) => {
    const updatedOptions = pollOptions.map((option) => ({
      ...option,
      checked: !option.checked && option.value === selectedId,
    }))
    setPollSelected(updatedOptions.find((opt) => opt.checked) ?? null)
    setPollOptions(updatedOptions)
    setNoPollSelected(updatedOptions.every((poll) => !poll.checked))
  }

  const castVote = () => {
    if (!user) return openModal()
    if (pollSelected && onVoted) {
      onVoted(pollSelected)
    }
  }

  return (
    <>
      <div className='relative flex min-h-screen w-screen flex-col items-center justify-center px-6 py-28'>
        <div className='absolute bottom-0 right-0 grid w-[70vh] grid-cols-2 gap-2'>
          <CircularTiles count={4} />
        </div>

        <div className='relative mx-auto flex w-full max-w-screen-md flex-col items-center justify-center space-y-12'>
          <div className='space-y-2'>
            <p className='text-center text-sm font-extrabold uppercase text-slate-400'>Daily Poll</p>
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
          {loading && <LoadingAnimation isLoading={loading} />}
          <div className='grid w-full grid-cols-2 gap-4 md:gap-8'>
            {pollOptions.map((poll) => (
              <div key={poll.label} className='col-span-2 md:col-span-1'>
                <Card checked={poll.checked} onChecked={() => handleChecked(poll.value)}>
                  <p className='inline-block text-8xl leading-none'>{poll.label}</p>
                </Card>
              </div>
            ))}
          </div>
          <div className='space-y-4'>
            {noPollSelected && <div className='text-center text-sm leading-none text-slate-500'>Select your favorite</div>}
            <button
              className={`button-outlined button-max ${noPollSelected ? 'button-disabled' : ''}`}
              disabled={noPollSelected || loading}
              onClick={castVote}
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
