// 'use client'

import { useTwitter } from '@/hooks/twitter/useTwitter'
import React, { useState } from 'react'
interface RegisterProps {
  onClose: () => void
}

export const AUTH_MSG = `🤫 I am authenticating with my Twitter account to cast my first encrypted vote with CRISP!

Visit https://crisp.gnosisguild.org/ to vote.

#FHE #ZKP #CRISP`

const RegisterModal: React.FC<RegisterProps> = ({ onClose }) => {
  const [showVerification, setShowVerification] = useState<boolean>(false)
  const [postUrl, setPostUrl] = useState<string>('')
  const { isLoading, handleTwitterPostVerification } = useTwitter()

  const handlePost = () => {
    if (!showVerification) {
      window.open(
        'https://twitter.com/intent/post?text=🤫%20I%20am%20authenticating%20with%20my%20Twitter%20account%20to%20cast%20my%20first%20encrypted%20vote%20with%20CRISP%21%0A%0A%23FHE%20%23ZKP%20%23CRISP',
        '_blank',
      )
      setShowVerification(true)
    }
  }

  const handlePostVerification = async () => {
    if (postUrl) {
      await handleTwitterPostVerification(postUrl)
      onClose()
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
            <input className='input' placeholder='Post URL' value={postUrl} onChange={({ target }) => setPostUrl(target.value)} />
          </fieldset>
          <div className='flex items-center justify-between'>
            <button className='button-outlined button-max' onClick={() => setShowVerification(false)} disabled={isLoading}>
              Back
            </button>
            <button className='button-primary button-max' onClick={handlePostVerification} disabled={isLoading}>
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
              validate ownership of your twitter account via a single post.
            </p>
          </div>
          <div className='space-y-2'>
            <p className='text-base font-extrabold uppercase text-slate-500'>WHAT HAPPENS NEXT?</p>
            <p className='text-lg  text-slate-600'>
              An address will be automatically generated and associated with your twitter account so you can easily authenticate and vote in
              future polls.
            </p>
          </div>
          <div className='flex items-center gap-4 max-md:flex-col'>
            <textarea rows={4} className='input h-auto' disabled value={AUTH_MSG} />
            <div className='hidden h-16 w-0.5 bg-slate-200 md:block' />
            <button className='button-primary md:button-max whitespace-nowrap max-md:w-full' onClick={handlePost}>
              Share post
            </button>
          </div>
        </>
      )}
    </div>
  )
}

export default RegisterModal
