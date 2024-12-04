import { cn } from '@/lib/utils'
import React from 'react'
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious
} from '@/components/ui/carousel'
import LazyImage from '@/components/lazy-image'
import { getTmdbImageUrl } from '@/utils/tmdb'
import { Credits, TVCredits } from '@/api'

export interface CreditsCarousel extends React.HTMLAttributes<HTMLDivElement> {
  credits: Credits | TVCredits
}

const CreditsCarousel = React.forwardRef<HTMLDivElement, CreditsCarousel>(
  ({ className, credits, ...props }, ref) => {
    return (
      <div className={cn('relative space-y-4', className)} ref={ref} {...props}>
        <Carousel className="relative">
          <CarouselPrevious className="absolute z-30 left-5" />
          <CarouselNext className="absolute z-30 right-5" />
          <CarouselContent className="px-6">
            {credits.cast.map((person) => (
              <CarouselItem
                key={person.id}
                title={person.name}
                className="basis-60 md:basis-56 lg:basis-60"
              >
                <div className="flex flex-col items-center justify-center text-center text-nowrap">
                  <LazyImage
                    src={getTmdbImageUrl(person.profile_path) || ''}
                    alt={person.name}
                    className="w-52 h-52 object-cover rounded-full"
                  />
                  <h3 className="text-foreground font-semibold text-md">
                    {person.name}
                  </h3>
                  <p className="text-foreground text-sm">{person.character}</p>
                </div>
              </CarouselItem>
            ))}
            {credits.crew.map((person) => (
              <CarouselItem
                key={person.id}
                title={person.name}
                className="basis-60 md:basis-56 lg:basis-60"
              >
                <div className="flex flex-col items-center justify-center text-center text-nowrap">
                  <LazyImage
                    src={getTmdbImageUrl(person.profile_path) || ''}
                    alt={person.name}
                    className="w-52 h-52 object-cover rounded-full"
                  />
                  <h3 className="text-foreground font-semibold text-md">
                    {person.name}
                  </h3>
                  <p className="text-foreground text-sm">{person.job}</p>
                </div>
              </CarouselItem>
            ))}
          </CarouselContent>
        </Carousel>
      </div>
    )
  }
)
CreditsCarousel.displayName = 'CreditsCarousel'

export default CreditsCarousel
