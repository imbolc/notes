# Multi-file replace

Replace all occurrences of `foo` by `bar` for every markdown file in the current and sub-folders:

```vim
:args **/*.md
:argdo %s/foo/bar/ge | update
```
