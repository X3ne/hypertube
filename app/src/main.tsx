import { StrictMode } from 'react'
import ReactDOM from 'react-dom/client'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { UserProvider } from '@/providers/userProvider'
import { SongProvider } from '@/providers/songProvider'
import { GradientProvider } from '@/providers/gradientProvider'
import './global.css'

import { routeTree } from './routeTree.gen'
import { PlaybackProvider } from './providers/playbackProvider'

const router = createRouter({ routeTree })

declare module '@tanstack/react-router' {
  interface Register {
    router: typeof router
  }
}

const queryClient = new QueryClient()

const rootElement = document.getElementById('root')!
if (!rootElement.innerHTML) {
  const root = ReactDOM.createRoot(rootElement)
  root.render(
    <StrictMode>
      <QueryClientProvider client={queryClient}>
        <GradientProvider>
          <SongProvider>
            <UserProvider>
              <PlaybackProvider>
                <RouterProvider router={router} />
              </PlaybackProvider>
            </UserProvider>
          </SongProvider>
        </GradientProvider>
      </QueryClientProvider>
    </StrictMode>
  )
}
