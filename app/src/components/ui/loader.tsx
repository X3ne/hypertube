import { cn } from "@/lib/utils"
import React from "react"

const Loader = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement>
>(({ className, ...props }, ref) => (
  <div
    ref={ref}
    className={cn(
      "animate-spin w-8 h-8 border-t-2 border-b-2 border-primary-foreground rounded-full",
      className
    )}
    {...props}
  />
))
Loader.displayName = "Loader"

export { Loader }
