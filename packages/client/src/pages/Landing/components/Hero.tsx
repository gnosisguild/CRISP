import React from 'react'
import Logo from '@/assets/icons/logo.svg'
import CircularTiles from '@/components/CircularTiles'
import { Link } from 'react-router-dom'
import { ArrowSquareOut, Keyhole, ListMagnifyingGlass, ShieldCheck } from '@phosphor-icons/react'
import { useVoteManagementContext } from '@/context/voteManagement'
import { pk_bytes } from '@/mocks/pk_key'
import { ethers, utils, providers } from 'ethers'

const contractAddress = '0x51Ec8aB3e53146134052444693Ab3Ec53663a12B'
const dummy = ''

const contractABI = [
  {
    inputs: [
      {
        internalType: 'bytes',
        name: '_encVote',
        type: 'bytes',
      },
    ],
    name: 'voteEncrypted',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function',
  },
]
const HeroSection: React.FC = () => {
  const { encryptVote } = useVoteManagementContext()

  const provider = new providers.JsonRpcProvider('https://sepolia.infura.io/v3/')
  const contractInterface = new utils.Interface(contractABI)
  const wallet = new ethers.Wallet(dummy, provider)

  const test = async () => {
    if (!provider) {
      console.log('no provider')
      return
    }
    const contract = new ethers.Contract(contractAddress, contractInterface, wallet)
    const voteEncrypted = await encryptVote(BigInt(1), new Uint8Array(pk_bytes))
    console.log('voteEncrypted', voteEncrypted?.toString())
    const tx = await contract.voteEncrypted(voteEncrypted)
    console.log('tx', tx)
  }

  return (
    <div className='relative flex min-h-screen w-screen items-center justify-center px-6'>
      <div className='absolute bottom-1 right-0 w-[40vh] space-y-2'>
        <CircularTiles count={2} />
      </div>
      <div className='relative mx-auto w-full max-w-screen-md space-y-12'>
        <div className='space-y-4'>
          <h3 className='text-3xl font-normal leading-none text-zinc-400'>Introducing</h3>
          <img src={Logo} alt='CRISP Logo' className='h-20' />
          <h4 className='w-full text-base leading-none text-slate-800/50'>Collusion-Resistant Impartial Selection Protocol</h4>
        </div>
        <ul className='space-y-3'>
          <li className='flex items-center space-x-2'>
            <Keyhole className='text-lime-600/80' size={32} />
            <div className='text-lg text-zinc-400'>
              <span className='mr-1 font-bold text-lime-600/80'>Private.</span>
              Voter privacy through advanced encryption.
            </div>
          </li>
          <li className='flex items-center space-x-2'>
            <ListMagnifyingGlass className='text-lime-600/80' size={32} />
            <div className='text-lg text-zinc-400'>
              <span className='mr-1 font-bold text-lime-600/80'>Reliable.</span>
              Verifiable results while preserving confidentiality.
            </div>
          </li>
          <li className='flex items-center space-x-2'>
            <ShieldCheck className='text-lime-600/80' size={32} />
            <div className='text-lg text-zinc-400'>
              <span className='mr-1 font-bold text-lime-600/80'>Equitable.</span>
              Robust safeguards against collusion and tampering.
            </div>
          </li>
        </ul>
        <div className='space-y-3'>
          {/* <Link to='/daily'> */}
          <button className='button-primary' onClick={test}>
            Try Demo
          </button>
          {/* </Link> */}
          <Link
            to='/about'
            className='inline-flex cursor-pointer items-center space-x-2 text-lime-600 duration-300 ease-in-out hover:opacity-70'
          >
            <ArrowSquareOut size={20} weight='bold' />
            <div>Learn more.</div>
          </Link>
        </div>
      </div>
    </div>
  )
}

export default HeroSection
