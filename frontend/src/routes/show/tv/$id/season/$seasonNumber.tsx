import * as React from 'react'
import { createFileRoute } from '@tanstack/react-router'
import { useQuery } from '@tanstack/react-query'
import api from '@/api'
import LazyImage from '@/components/lazy-image'
import { getTmdbImageUrl, getTmdbPlaceholderImageUrl } from '@/utils/tmdb'

export const Route = createFileRoute('/show/tv/$id/season/$seasonNumber')({
  component: TvSeason
})

function TvSeason() {
  const { id, seasonNumber } = Route.useParams()

  const { data: season } = useQuery({
    queryKey: ['season', id, seasonNumber],
    queryFn: async () => {
      return (await api.getTvSeason(Number(id), Number(seasonNumber))).data
    },
    refetchOnWindowFocus: false
  })

  return (
    <div className="pt-28 px-36">
      {season && season.episodes ? (
        <div>
          <div className="flex flex-wrap">
            {season.episodes.map((episode) => {
              if (new Date(episode.air_date) > new Date()) return
              return (
                <div key={episode.id} className="flex flex-col gap-2 w-1/5 p-2">
                  <LazyImage
                    src={getTmdbImageUrl(episode.still_path) || ''}
                    placeholder={
                      getTmdbPlaceholderImageUrl(episode.still_path) || ''
                    }
                    alt={episode.name}
                    className="w-full rounded-md"
                  />
                  <div>
                    <h2>{episode.name}</h2>
                    <p className="text-foreground/70 text-sm">
                      Episode {episode.episode_number}
                    </p>
                  </div>
                </div>
              )
            })}
          </div>
        </div>
      ) : (
        <p>Loading...</p>
      )}
    </div>
  )
}
