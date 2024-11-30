import { useUser } from '@/hooks/useUser'
import NavBarDesktop from './desktop'
import NavBarMobile from './mobile'

const NavBar = () => {
  const { user, logout } = useUser()

  return (
    <nav>
      <NavBarDesktop
        user={null}
        logout={logout}
        className="invisible md:visible"
      />
      {/* <NavBarMobile user={null} className="visible md:invisible" /> */}
    </nav>
  )
}

export default NavBar
