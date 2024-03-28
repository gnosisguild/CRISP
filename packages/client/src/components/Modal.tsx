// Modal.tsx
import React, { useEffect, useRef, useCallback, FC } from 'react'

interface ModalProps {
  show: boolean
  onClose: () => void
  children: React.ReactNode
}

const Modal: FC<ModalProps> = ({ show, onClose, children }) => {
  const modalRef = useRef<HTMLDivElement>(null)

  const closeModal = (e: React.MouseEvent<HTMLDivElement>) => {
    if (modalRef.current === e.target) {
      onClose()
    }
  }

  const keyPress = useCallback(
    (e: KeyboardEvent) => {
      if (e.key === 'Escape' && show) {
        onClose()
      }
    },
    [show, onClose],
  )

  useEffect(() => {
    document.addEventListener('keydown', keyPress)
    return () => document.removeEventListener('keydown', keyPress)
  }, [keyPress])

  if (!show) return null

  return (
    <div className='fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-20' onClick={closeModal} ref={modalRef}>
      <div className='shadow-modal relative max-h-[672px] w-full max-w-[900px] rounded-[24px] border-2 border-twilight-blue-200 bg-white-900 p-12'>
        {children}
        <button className='absolute right-0 top-0 mr-8 mt-8' onClick={onClose}>
          <div className='close-icon' />
        </button>
      </div>
    </div>
  )
}

export default Modal
