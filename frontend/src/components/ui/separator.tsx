import { cn } from '@/lib/utils'
import React from 'react'

export interface SeparatorProps extends React.HTMLAttributes<HTMLDivElement> {
  header?: string
}

const Separator = React.forwardRef<HTMLDivElement, SeparatorProps>(
  ({ className, header, ...props }, ref) => {
    return (
      <div className={cn('relative space-y-4', className)} ref={ref} {...props}>
        <div className="flex justify-center items-center gap-4">
          <span className="bg-primary/15 h-[1px] w-full"></span>
          {header && (
            <>
              <h2 className="text-lg font-semibold">{header}</h2>
              <span className="bg-primary/15 h-[1px] w-full"></span>
            </>
          )}
        </div>
      </div>
    )
  }
)
Separator.displayName = 'Separator'

export default Separator
