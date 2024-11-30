import api from '@/api'
import NavBar from '@/components/app/navbar/navbar'
import { Toaster } from '@/components/ui/toaster'
import { useUser } from '@/hooks/useUser'
import { useQuery } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'
import {
  createRootRoute,
  Outlet,
  useLocation,
  useNavigate
} from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { useEffect } from 'react'

const AllowedRoutes = ['/auth/login', '/auth/plex/callback', '/']

export const Route = createRootRoute({
  component: () => {
    const { user } = useUser()
    const navigation = useNavigate()
    const location = useLocation()

    const controller = new AbortController()
    const signal = controller.signal

    setTimeout(() => {
      controller.abort()
    }, 2000)

    // const { isError } = useQuery({
    //   queryKey: ['health'],
    //   queryFn: async () => {
    //     return await api.health({
    //       signal
    //     })
    //   },
    //   retry: false
    // })

    // useEffect(() => {
    //   if (isError && location.pathname !== '/error') {
    //     navigation({
    //       to: '/error'
    //     })
    //   } else if (!isError && location.pathname === '/error') {
    //     navigation({
    //       to: '/'
    //     })
    //   }
    // }, [location, isError])

    // useEffect(() => {
    //   if (isError) return
    //   if (!user) {
    //     if (!AllowedRoutes.includes(location.pathname)) {
    //       navigation({
    //         to: '/auth/login'
    //       })
    //     }
    //   } else if (!user.setup_completed) {
    //     navigation({
    //       to: '/setup'
    //     })
    //   }
    // }, [user, location])

    return (
      <>
        <div className="w-screen min-h-dvh">
          <NavBar />
          <Outlet />
        </div>
        <TanStackRouterDevtools />
        <ReactQueryDevtools initialIsOpen={false} />
        <Toaster />
      </>
    )
  }
})
