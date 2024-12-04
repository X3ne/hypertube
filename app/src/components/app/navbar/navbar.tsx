import { useUser } from '@/hooks/useUser'
import NavBarDesktop from './desktop'

const NavBar = () => {
  const { user, logout } = useUser()

  return (
    <nav>
      <NavBarDesktop
        user={user}
        logout={logout}
        className="invisible md:visible"
      />
      {/* <NavBarMobile users={null} className="visible md:invisible" /> */}
    </nav>
  )
}

export default NavBar
