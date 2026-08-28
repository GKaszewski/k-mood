import { buttonVariants } from "@/components/ui/button"
import { cn } from "@/lib/utils"
import type { VariantProps } from "class-variance-authority"

type ButtonLook = VariantProps<typeof buttonVariants>

/**
 * Button styling for something that is not a button. Navigation is a link, and
 * passing a `<Link>` to `Button` renders an anchor where a native button was
 * declared — this dresses the link instead, so `to` and `params` keep the
 * router's own type checking.
 */
export function buttonLook(look: ButtonLook = {}, className?: string): string {
  return cn(buttonVariants(look), className)
}
