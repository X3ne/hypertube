import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger
} from '@/components/ui/dropdown-menu'
import { cn } from '@/lib/utils'
import { Link } from '@tanstack/react-router'
import { useEffect, useState } from 'react'
import {
  LuBell,
  LuChevronDown,
  LuLogOut,
  LuSearch,
  LuSettings,
  LuUser,
  LuX
} from 'react-icons/lu'
import React from 'react'
import { Button } from '@/components/ui/button'
import { useMutation } from '@tanstack/react-query'
import api from '@/api'
import { useDebounce } from '@/hooks/useDebounce'
import { debounce } from '@/utils'
import LazyImage from '@/components/lazy-image'
import {
  getTmdbImageUrl,
  getTmdbPlaceholderImageUrl
} from '../../../utils/tmdb'

interface INavBarDesktopProps extends React.HTMLAttributes<HTMLDivElement> {
  user: null
  logout: () => void
}

const UserSection = ({ user, logout }: INavBarDesktopProps) => {
  const [isDropdownOpen, setIsDropdownOpen] = useState(false)

  return (
    <>
      <LuBell className="w-5 h-5" />
      <DropdownMenu onOpenChange={(isOpen) => setIsDropdownOpen(isOpen)}>
        <DropdownMenuTrigger asChild>
          <div className="flex items-center gap-1 cursor-pointer">
            <Avatar>
              <AvatarImage src={user.avatar} alt="Profile picture" />
              <AvatarFallback>
                {user.username.slice(0, 2).toUpperCase()}
              </AvatarFallback>
            </Avatar>
            <LuChevronDown
              className={cn(
                'w-5 h-5 transition-all',
                isDropdownOpen ? 'transform rotate-180' : 'transform rotate-0'
              )}
            />
          </div>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="mr-8 mt-1">
          <DropdownMenuLabel>
            <div className="flex gap-4">
              <Avatar>
                <AvatarImage src={user.avatar} alt="Profile picture" />
                <AvatarFallback>
                  {user.username.slice(0, 2).toUpperCase()}
                </AvatarFallback>
              </Avatar>
              <div className="flex flex-col">
                <span className="text-md truncate">Hello, {user.username}</span>
                <span className="text-foreground/80 truncate">
                  {user.email}
                </span>
              </div>
            </div>
          </DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuItem>
            <LuUser className="w-5 h-5 mr-2" />
            <span>Profile</span>
          </DropdownMenuItem>
          <DropdownMenuItem>
            <LuSettings className="w-5 h-5 mr-2" />
            <span>Settings</span>
          </DropdownMenuItem>
          <DropdownMenuItem onClick={logout}>
            <LuLogOut className="w-5 h-5 mr-2" />
            <span>Logout</span>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  )
}

const SearchAera = ({
  opened,
  setOpened
}: {
  opened: boolean
  setOpened: (value: boolean) => void
}) => {
  // TODO: Autofocus input on open
  const {
    data: searchResults,
    mutate: search,
    reset
  } = useMutation({
    mutationKey: ['search'],
    mutationFn: async (query: string) => {
      return (
        await api.searchShows({
          query
        })
      ).data
    }
  })

  const debouncedSearch = debounce((value: string) => {
    if (!value) {
      reset()
      return
    }

    search(value)
  }, 500)

  return (
    <div
      className={cn(
        'flex fixed top-0 left-0 z-50 w-screen h-screen bg-secondary/80 backdrop-blur-sm flex-col gap-8 items-center justify-center transition-all duration-300',
        opened
          ? 'opacity-100 pointer-events-auto'
          : 'opacity-0 pointer-events-none'
      )}
    >
      <LuX
        className="absolute top-0 right-0 w-8 h-8 m-4 cursor-pointer"
        onClick={() => setOpened(false)}
      />
      <input
        type="text"
        placeholder="Search..."
        className="w-96 h-12 px-4 py-2 rounded-full ring-0 outline-none text-secondary transition-all duration-300"
        onChange={(e) => debouncedSearch(e.target.value)}
      />
      <div className="flex flex-col gap-4 w-[1000px] h-[688px]">
        {searchResults && (
          <>
            <h1 className="text-2xl font-semibold">TV Shows</h1>
            <div className="flex gap-2">
              {searchResults?.tv.slice(0, 5).map((tv) => (
                <Link
                  key={tv.id}
                  to={`/show/tv/${tv.id}`}
                  onClick={() => setOpened(false)}
                  className="group"
                >
                  <div className="relative rounded-md overflow-hidden">
                    <LazyImage
                      src={getTmdbImageUrl(tv.poster_path) || ''}
                      placeholder={
                        getTmdbPlaceholderImageUrl(tv.poster_path) || ''
                      }
                      alt={tv.name}
                      className="w-48 h-72 object-cover"
                    />
                    <div
                      className={cn(
                        'absolute top-0 left-0 flex justify-center items-center h-full w-full p-4 text-center font-semibold opacity-0 bg-secondary/80 backdrop-blur-sm transition-opacity duration-300',
                        'group-hover:opacity-100'
                      )}
                    >
                      <span>{tv.name}</span>
                    </div>
                  </div>
                </Link>
              ))}
            </div>
            <h1 className="text-2xl font-semibold">Movies</h1>
            <div className="flex gap-2">
              {searchResults?.movies.slice(0, 5).map((movie) => (
                <Link
                  key={movie.id}
                  to={`/show/movie/${movie.id}`}
                  onClick={() => setOpened(false)}
                  className="group"
                >
                  <div className="relative rounded-md overflow-hidden">
                    <LazyImage
                      src={getTmdbImageUrl(movie.poster_path) || ''}
                      placeholder={
                        getTmdbPlaceholderImageUrl(movie.poster_path) || ''
                      }
                      alt={movie.title}
                      className="w-48 h-72 object-cover"
                    />
                    <div
                      className={cn(
                        'absolute top-0 left-0 flex justify-center items-center h-full w-full p-4 text-center font-semibold opacity-0 bg-secondary/80 backdrop-blur-sm transition-opacity duration-300',
                        'group-hover:opacity-100'
                      )}
                    >
                      <span>{movie.title}</span>
                    </div>
                  </div>
                </Link>
              ))}
            </div>
          </>
        )}
      </div>
    </div>
  )
}

const NavBarDesktop = ({ user, logout, className }: INavBarDesktopProps) => {
  const [isTop, setIsTop] = useState(true)
  const [isSearchBarExpended, setIsSearchBarExpended] = useState(false)

  useEffect(() => {
    const handleScroll = () => {
      setIsTop(window.scrollY === 0)
    }

    window.addEventListener('scroll', handleScroll)
    return () => {
      window.removeEventListener('scroll', handleScroll)
    }
  })

  return (
    <>
      <SearchAera
        opened={isSearchBarExpended}
        setOpened={setIsSearchBarExpended}
      />
      <div
        className={cn(
          className,
          'fixed w-full px-8 py-4 left-0 top-0 flex flex-row justify-between items-center bg-transparent text-foreground z-30 transition-colors duration-500',
          !isTop && 'bg-background/40 backdrop-blur-md'
        )}
      >
        <div className="space-x-4"></div>
        <div>
          <Link to="/" className="text-2xl font-semibold">
            Hypertube
          </Link>
        </div>
        <div className="flex items-center gap-4">
          <div className="relative">
            <LuSearch
              className="w-5 h-5 cursor-pointer"
              onClick={() => setIsSearchBarExpended(true)}
            />
          </div>
          {user ? (
            <UserSection user={user} logout={logout} />
          ) : (
            <Link to="/login" className="flex items-center gap-1">
              <Button variant={'default'}>Login</Button>
            </Link>
          )}
        </div>
      </div>
    </>
  )
}

export default NavBarDesktop
