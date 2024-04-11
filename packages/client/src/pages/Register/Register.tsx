import React from 'react'
interface RegisterProps {
  onClose: () => void
}
//

const RegisterModal: React.FC<RegisterProps> = ({ onClose }) => {
  const handleSubmit = () => {
    window.open(
      'https://twitter.com/intent/tweet?text=I%20authenticate%20that%20this%20Twitter%20account%20is%20owned%20by%20me.%20%23crisp%20%23dailypoll_13',
      '_blank',
    )
  }
  return (
    <div className='space-y-10'>
      <div className='space-y-2'>
        <p className='text-sm font-extrabold uppercase text-zinc-900'>register</p>
        <h2 className='text-h2 font-bold text-twilight-blue-900'>Verify your account via Twitter</h2>
      </div>
      <div className='space-y-2'>
        <p className='text-base font-extrabold uppercase text-gray-900'>why am i doing this</p>
        <p className=' text-xl  text-twilight-blue-900'>
          Since this is a simple single-use web app, we&apos;re creating an easy-to-use authentication system that only requires you to
          validate ownership of your Twitter account via a single post.
        </p>
      </div>
      <div className='space-y-2'>
        <p className='text-base font-extrabold uppercase text-gray-900'>WHAT HAPPENS NEXT?</p>
        <p className=' text-xl  text-twilight-blue-900'>
          An address will be automatically generated and associated with your twitter account so you can easily authenticate and vote in
          future daily polls.
        </p>
      </div>

      <textarea
        rows={2}
        className='input h-auto'
        disabled
        value='I authenticate that this twitter account is owned by me. #crisp #dailypoll_13'
      />
      <div className='flex items-center justify-between'>
        <button className='button-outlined button-max' onClick={onClose}>
          cancel
        </button>
        <button className='button button-max' onClick={handleSubmit}>
          submit
        </button>
      </div>
    </div>
  )
}

export default RegisterModal
