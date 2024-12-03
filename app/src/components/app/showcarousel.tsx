import { cn } from '@/lib/utils'
import React from 'react'
import { Button } from '@/components/ui/button'
import { LuArrowRight } from 'react-icons/lu'
import { Card, CardContent, CardFooter } from '@/components/ui/card'
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious
} from '@/components/ui/carousel'
import { Link } from '@tanstack/react-router'
import LazyImage from '@/components/lazy-image'
import { getTmdbImageUrl, getTmdbPlaceholderImageUrl } from '@/utils/tmdb'
import { Movie, TV } from '@/api'
import { Popover, PopoverContent } from '@/components/ui/popover'

export interface CarouselProps<S> extends React.HTMLAttributes<HTMLDivElement> {
  header?: string
  items: S[]
}

export interface ShowCarouselProps<S>
  extends React.HTMLAttributes<HTMLDivElement> {
  header?: string
  items: S[]
  linkPrefix: string
  getTitle: (item: S) => string
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const ShowCarousel = React.forwardRef<HTMLDivElement, ShowCarouselProps<any>>(
  ({ className, items, header, linkPrefix, getTitle, ...props }, ref) => {
    return (
      <div className={cn('relative space-y-4', className)} ref={ref} {...props}>
        {header && (
          <div className="flex justify-between items-center">
            <h2 className="text-foreground font-semibold text-xl">{header}</h2>
            <Button variant="outline" className="gap-1 px-8">
              See more
              <LuArrowRight className="h-4 w-4" />
            </Button>
          </div>
        )}
        <Carousel className="relative">
          <CarouselPrevious className="absolute z-30 left-5" />
          <CarouselNext className="absolute z-30 right-5" />
          <CarouselContent>
            {items.map((item) => (
              <CarouselItem
                key={item.id}
                title={getTitle(item)}
                className="basis-2/6 md:basis-1/6 lg:basis-1/6 2xl:basis-64"
              >
                <Link to={`${linkPrefix}/${item.id}`} className="w-32">
                  <Card className="bg-transparent border-transparent text-foreground space-y-2 select-none">
                    <CardContent className="p-0">
                      <LazyImage
                        src={getTmdbImageUrl(item.poster_path) || ''}
                        placeholder={
                          getTmdbPlaceholderImageUrl(item.poster_path) || ''
                        }
                        alt={getTitle(item)}
                        className=" rounded-md"
                      />
                    </CardContent>
                    <CardFooter className="p-0">
                      <div className="flex items-center gap-2 w-full h-fit">
                        <h1 className="font-medium line-clamp-1">
                          {getTitle(item)}
                        </h1>
                        <div>
                          <LuArrowRight />
                        </div>
                      </div>
                    </CardFooter>
                  </Card>
                </Link>
              </CarouselItem>
            ))}
          </CarouselContent>
        </Carousel>
      </div>
    )
  }
)
ShowCarousel.displayName = 'ShowCarousel'

const TvCarousel = React.forwardRef<HTMLDivElement, CarouselProps<TV>>(
  (props, ref) => (
    <ShowCarousel
      ref={ref}
      {...props}
      linkPrefix="/show/tv"
      getTitle={(show) => show.name}
    />
  )
)

const MovieCarousel = React.forwardRef<HTMLDivElement, CarouselProps<Movie>>(
  (props, ref) => (
    <ShowCarousel
      ref={ref}
      {...props}
      linkPrefix="/show/movie"
      getTitle={(show) => show.title}
    />
  )
)

export { TvCarousel, MovieCarousel }
