## Responsive left <=> right

The elements stick to the borders and wraps when there's not enough space.

```css
display: flex;
justify-content: space-between;
flex-wrap: wrap;
gap: 1em;
```

### Example

<div style="display: flex; justify-content: space-between; flex-wrap: wrap; gap: 1em">
    <div style="background: lightgreen; padding: 1em 15em">foo</div>
    <div style="background: lightblue; padding: 1em 15em">bar</div>
</div>
