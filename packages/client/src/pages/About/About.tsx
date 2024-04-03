import React from 'react'
import CircleIcon from '../../assets/icons/caretCircle.svg'

const About: React.FC = () => {
  return (
    <div className='my-28 flex w-screen flex-col items-center justify-center space-y-12'>
      <h1 className='text-h1 font-bold text-twilight-blue-900'>About CRISP</h1>
      <div className='h-[716px] w-full max-w-[900px] space-y-10 rounded-[24px] border-2 border-twilight-blue-200 bg-white-900 p-12 shadow-modal'>
        <div className='space-y-4'>
          <p className='text-base font-extrabold uppercase text-twilight-blue-500'>what is crisp?</p>
          <div className='space-y-2'>
            <p className='text-xl leading-8 text-twilight-blue-900'>
              CRISP (Collusion-Resistant Impartial Selection Protocol) is a groundbreaking component of the Enclave protocol, focused on
              revolutionizing privacy and security in digital decision-making. It leverages advanced technologies like Fully Homomorphic
              Encryption (FHE), threshold cryptography, and zero-knowledge proofs (ZKPs) to enable secure, anonymous voting. This protocol
              ensures that the integrity of each vote is maintained without compromising the voter's privacy, making it a powerful tool for
              governance and decision-making applications.
            </p>
            <div className='flex cursor-pointer items-center space-x-2'>
              <p className='text-green-light underline'>See what&apos;s happening under the hood</p>
              <img src={CircleIcon} className='h-[18] w-[18]' />
            </div>
          </div>
        </div>
        <div className='space-y-4'>
          <p className='text-base font-extrabold uppercase text-twilight-blue-500'>why is this important?</p>
          <p className='text-xl leading-8 text-twilight-blue-900'>
            In a digital age marked by increasing concerns over privacy, security, and the integrity of information, CRISP emerges as a
            crucial innovation. By protecting against collusion, mitigating vulnerabilities in governance, and preserving the
            confidentiality of data, CRISP fosters a secure and impartial environment for decision-making. It empowers users and
            organizations to participate in governance and other sensitive processes with assurance, promoting fairness, transparency, and
            trust in digital systems.
          </p>
        </div>
      </div>

      <button className='button-outlined button-max'>view whitepaper</button>
    </div>
  )
}

export default About
