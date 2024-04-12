// NavMenu.tsx
import React, { useEffect, useRef, useState } from 'react'
import CalendarIcon from '../assets/icons/calendarCheck.svg'
import CheckIcon from '../assets/icons/check.svg'
import NotebookIcon from '../assets/icons/notebook.svg'
import LogoutIcon from '../assets/icons/logout.svg'
import { useNavigate } from 'react-router-dom'
//Icons
import Avatar from '../assets/images/exampleAvatar.svg'
import ArrowRight from '../assets/icons/arrowRight.svg'

interface NavMenuProps {}

const NAV_MENU_OPTIONS = [
  {
    name: 'Daily Poll',
    icon: CalendarIcon,
    path: '/daily',
  },
  {
    name: 'Historic Polls',
    icon: CheckIcon,
    path: '/historic',
  },
  {
    name: 'About',
    icon: NotebookIcon,
    path: '/about',
  },
]

const NavMenu: React.FC<NavMenuProps> = () => {
  const navigate = useNavigate()
  const menuRef = useRef<HTMLDivElement>(null)
  const [isOpen, setIsOpen] = useState<boolean>(false)
  const buttonRef = useRef<HTMLButtonElement>(null)

  const handleClickOutside = (event: MouseEvent) => {
    if (
      isOpen &&
      menuRef.current &&
      !menuRef.current.contains(event.target as Node) &&
      !buttonRef.current?.contains(event.target as Node)
    ) {
      setIsOpen(false)
    }
  }

  const toggleMenu = (event: React.MouseEvent<HTMLButtonElement>) => {
    event.stopPropagation()
    setIsOpen(!isOpen)
  }

  useEffect(() => {
    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside)
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside)
    }
  }, [isOpen])

  const handleNavigation = (path: string) => {
    navigate(path)
    return setIsOpen(!isOpen)
  }

  const handleLogout = () => {
    navigate('/')
    return setIsOpen(!isOpen)
  }

  return (
    <div className='relative'>
      <button
        ref={buttonRef}
        onClick={toggleMenu}
        className='hover:bg-white bg-white/60 flex items-center justify-between space-x-1 rounded-lg border-2 px-2 py-1 duration-300 ease-in-out'
      >
        <img src={Avatar} />
        <p className='text-xs font-bold'>@juliopavila</p>
        <img src={ArrowRight} className={isOpen ? '-rotate-90' : ''} />
      </button>

      <div
        ref={menuRef}
        className={`bg-white absolute right-0 mt-4 w-40 transform rounded-lg border-2 border-slate-600/10 p-4  shadow-md ${
          isOpen ? 'scale-100' : 'scale-0'
        }`}
      >
        <div className='space-y-2'>
          {NAV_MENU_OPTIONS.map(({ name, path, icon }) => (
            <div key={name} className='flex cursor-pointer space-x-2 rounded p-1 hover:bg-gray-100' onClick={() => handleNavigation(path)}>
              <img src={icon} />
              <p className='block rounded-md text-sm font-semibold '>{name}</p>
            </div>
          ))}
          <div className='border-t-2'>
            <div className='mt-2 flex cursor-pointer space-x-2 rounded  p-1 hover:bg-gray-100' onClick={handleLogout}>
              <img src={LogoutIcon} />
              <p className='block rounded-md text-sm font-semibold '>Logout</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

export default NavMenu
