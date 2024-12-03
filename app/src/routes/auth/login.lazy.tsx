import api from '@/api'
import Logo from '@/assets/logo'
import { Button } from '@/components/ui/button'
import { useUser } from '@/hooks/useUser'
import { useMutation } from '@tanstack/react-query'
import { createLazyFileRoute, useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'

export const Route = createLazyFileRoute('/auth/login')({
  component: () => {
    const { user } = useUser()
    const navigation = useNavigate()

    useEffect(() => {
      if (user) {
        navigation({
          to: '/',
        })
      }
    }, [user])

    const { mutate: getPlexAuthUri } = useMutation({
      mutationFn: async () => await api.plexLogin(),
      onSuccess: ({ data }) => {
        window.location.href = data.auth_url
      },
    })

    return (
      <div className="flex flex-col justify-center items-center min-h-screen space-y-6">
        <Logo height={82} />
        <h1 className="text-5xl font-bold">Hypertube</h1>
        <p className="text-foreground/80">Login with Plex to get started</p>
        <Button onClick={() => getPlexAuthUri()}>Login with Plex</Button>
      </div>
    )
  },
})
