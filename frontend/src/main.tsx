import { StrictMode } from 'react'
import ReactDOM from 'react-dom/client'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { UserProvider } from '@/providers/userProvider'
import { SongProvider } from '@/providers/songProvider'
import { GradientProvider } from '@/providers/gradientProvider'
import './global.css'

import { routeTree } from './routeTree.gen'

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
              <RouterProvider router={router} />
            </UserProvider>
          </SongProvider>
        </GradientProvider>
      </QueryClientProvider>
    </StrictMode>
  )
}