import api from '@/api'
import { Button } from '@/components/ui/button'
import { Loader } from '@/components/ui/loader'
import { useMutation, useQueryClient } from '@tanstack/react-query'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'

type CallbackParams = {
  id: string
  code: string
}

export const Route = createFileRoute('/auth/plex/callback')({
  validateSearch: (search: Record<string, unknown>): CallbackParams => {
    return {
      id: search.id as string,
      code: search.code as string
    }
  },
  component: () => {
    const navigate = useNavigate()
    const { id, code } = Route.useSearch()
    const queryClient = useQueryClient()

    const {
      mutate: callbackQuery,
      isPending,
      isError,
      error
    } = useMutation({
      mutationFn: async () =>
        await api.plexCallback(
          {
            id,
            code
          },
          {
            credentials: 'include'
          }
        ),
      onSuccess: () => {
        queryClient.invalidateQueries({ queryKey: ['user'] })
        setTimeout(async () => {
          navigate({
            to: '/'
          })
        }, 2000)
      },
      onError: (err) => {
        console.error(err)
      }
    })

    useEffect(() => {
      callbackQuery()
    }, [id, code])

    return (
      <div className="w-full h-screen flex flex-col items-center justify-center">
        {isPending && !isError && (
          <div className="text-center space-y-4">
            <h1 className="text-2xl font-bold">Authenticating...</h1>
            <p className="text-md text-foreground">
              Please wait while we authenticate you with Plex, you will be
              redirected shortly.
            </p>
            <Loader className="mx-auto" />
          </div>
        )}

        {!isPending && !isError && (
          <div className="text-center space-y-4">
            <p className="text-2xl font-bold mb-4">
              Authentication successful!
            </p>
            <p className="text-md text-foreground">
              Not redirected? Click the button below.
            </p>
            <Button variant="outline" onClick={() => navigate({ to: '/' })}>
              Go home
            </Button>
          </div>
        )}

        {isError && (
          <div className="text-center space-y-4">
            <p className="text-2xl font-bold mb-4">Authentication failed!</p>
            <p className="text-md text-foreground">{error.message}</p>
            <Button variant="outline" onClick={() => navigate({ to: '/' })}>
              Go home
            </Button>
          </div>
        )}
      </div>
    )
  }
})
