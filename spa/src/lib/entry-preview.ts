const MOST_CHARACTERS = 160

/** A note trimmed to something that fits on a card. */
export function contentPreview(content: string): string {
  const trimmed = content.trim()

  return trimmed.length > MOST_CHARACTERS
    ? `${trimmed.slice(0, MOST_CHARACTERS)}…`
    : trimmed
}
