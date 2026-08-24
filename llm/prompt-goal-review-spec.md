# Goal: review a spec implementation

Reviewer models:

```csv
model,reasoning effort
gpt-5.6-luna,high
gpt-5.6-luna,xhigh
gpt-5.6-terra,high
gpt-5.6-terra,xhigh
gpt-5.6-sol,high
gpt-5.6-sol,xhigh
```

For each reviewer model run the loop:

- Run another agent using the following command and wait for the result (it
  could take quite a bit - tens of minutes) :

  ```sh
  codex \
      --sandbox read-only \
      --model <reviewer model> \
      -c 'review_model="<reviewer model>"' \
      -c 'model_reasoning_effort="<reviewer reasoning effort>"' \
      review - 2>/dev/null <<'EOF'
  Review `./crates/foo/SPEC.md`
  Don't review or validate the code, only the spec.
  Don't run checks or tests, assume they all valid.
  EOF
  ```

- Evaluate its review. For each valuable finding, assign a severity: high,
  medium or low.
- Address every valuable finding of the review
- If you made changes commit them without asking for permission
- Update the streak based on the highest-severity valuable finding:
  - high: reset streak to 0
  - medium: leave streak unchanged
  - low or no valuable findings: increment streak

- Write into `./var/review.csv` (keep it gitignored):

  - timestamp
  - model used (e.g. luna-high)
  - number of valuable findings
  - highest severity
  - streak

- Repeat until the streak reaches 3, then start the loop using the next reviewer
  model
