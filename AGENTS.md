# Personal notes

## Markdown rules:

- maximal conciseness
- maximal structurization
- informal tone
- use abbreviations freely

## Note review

When you asked to review the note, check it for:

- typos
- facts correctness (if not sure, follow links to sources if a note has ones,
  otherwise search)
- missing crucial points
- better wording keeping conciseness

Skip anything that's fine. If it's not an issue, don't mention it. Don't
describe what you checked. If everything looks good, just say so.

List issues with unique IDs. Group issues by the categories above.

### Web scraping

If you're need to scrape pages, e.g. `curl ... | head ...`: save the page first
`curl ... > /tmp/...` and only then scrape them `head ...` to minimize network
requests.
