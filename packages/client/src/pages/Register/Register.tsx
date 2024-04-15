'use client'

import React, { useState } from 'react'
interface RegisterProps {
  onClose: () => void
}

const RegisterModal: React.FC<RegisterProps> = ({ onClose }) => {
  const [showVerification, setShowVerification] = useState<boolean>(false)
  const handleSubmit = () => {
    if (!showVerification) {
      window.open(
        'https://warpcast.com/~/compose?text=I%20authenticate%20that%20this%20Farcaster%20account%20is%20owned%20by%20me.%20#crisp%20#dailypoll_13',
        '_blank',
      )
      setShowVerification(true)
    }
  }
  return (
    <div className='space-y-10'>
      {showVerification ? (
        <>
          <div className='space-y-2'>
            <p className='text-sm font-extrabold uppercase text-slate-400'>Submit Post URL</p>
            <h3 className='font-bold text-slate-600'>Share the link to your post</h3>
          </div>
          <fieldset className='space-y-2'>
            <label className='text-base font-extrabold uppercase text-slate-500'>Post URL</label>
            <input className='input' placeholder='Post URL'></input>
          </fieldset>
          <div className='flex items-center justify-between'>
            <button className='button-outlined button-max' onClick={() => setShowVerification(false)}>
              Back
            </button>
            <button className='button-primary button-max' onClick={handleSubmit}>
              submit
            </button>
          </div>
        </>
      ) : (
        <>
          <div className='space-y-2'>
            <p className='text-sm font-extrabold uppercase text-slate-400'>register</p>
            <h2 className='text-h2 font-bold text-slate-600'>Verify your account</h2>
          </div>
          <div className='space-y-2'>
            <p className='text-base font-extrabold uppercase text-slate-500'>why am i doing this</p>
            <p className='text-lg  text-slate-600'>
              Since this is a simple single-use web app, we&apos;re creating an easy-to-use authentication system that only requires you to
              validate ownership of your Farcaster account via a single post.
            </p>
          </div>
          <div className='space-y-2'>
            <p className='text-base font-extrabold uppercase text-slate-500'>WHAT HAPPENS NEXT?</p>
            <p className='text-lg  text-slate-600'>
              An address will be automatically generated and associated with your twitter account so you can easily authenticate and vote in
              future daily polls.
            </p>
          </div>
          <div className='flex items-center space-x-4'>
            <textarea
              rows={2}
              className='input h-auto'
              disabled
              value='I authenticate that this twitter account is owned by me. #crisp #dailypoll_13'
            />
            <div className='h-16 w-0.5 bg-slate-200' />
            <button className='button-primary button-max whitespace-nowrap' onClick={handleSubmit}>
              Share post
            </button>
          </div>
        </>
      )}
    </div>
  )
}

export default RegisterModal
