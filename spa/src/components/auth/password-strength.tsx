import { cn } from "@/lib/utils"

/**
 * The server accepts any password, so this is guidance rather than a rule it
 * will enforce — but a journal is worth a password that is not "abc".
 */
const SHORTEST_SENSIBLE = 10

interface Requirement {
  describes: string
  met: (password: string) => boolean
}

const REQUIREMENTS: Requirement[] = [
  {
    describes: `at least ${SHORTEST_SENSIBLE} characters`,
    met: (password) => password.length >= SHORTEST_SENSIBLE,
  },
  {
    describes: "a letter and a digit, or a long passphrase",
    met: (password) =>
      (/\p{L}/u.test(password) && /\d/.test(password)) || password.length >= 20,
  },
]

export function isStrongEnough(password: string): boolean {
  return REQUIREMENTS.every((requirement) => requirement.met(password))
}

export function PasswordStrength({ password }: { password: string }) {
  if (!password) {
    return (
      <p id="password-strength" className="text-xs text-muted-foreground">
        Use {SHORTEST_SENSIBLE} characters or more.
      </p>
    )
  }

  return (
    <ul id="password-strength" className="flex list-none flex-col gap-0.5 p-0">
      {REQUIREMENTS.map((requirement) => {
        const met = requirement.met(password)

        return (
          <li
            key={requirement.describes}
            className={cn(
              "text-xs",
              met ? "text-muted-foreground" : "text-destructive"
            )}
          >
            <span aria-hidden="true">{met ? "✓" : "•"}</span>{" "}
            {requirement.describes}
            <span className="sr-only">{met ? " — met" : " — not yet met"}</span>
          </li>
        )
      })}
    </ul>
  )
}
