import { cn } from '@/lib/utils'
import { LuHome } from 'react-icons/lu'
import React from 'react'
import { User } from '@/api'

interface INavBarMobileProps extends React.HTMLAttributes<HTMLDivElement> {
  user: User
}

const NavBarMobile = ({ className }: INavBarMobileProps) => {
  return (
    <div
      className={cn(
        className,
        'fixed flex justify-center items-center w-full py-4 bg-background text-foreground bottom-0 z-40'
      )}
    >
      <LuHome />
    </div>
  )
}

export default NavBarMobile
