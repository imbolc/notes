# Goal: review a spec implementation

This request runs in the goal mode. That means I won't be there to allow
permission requests. Bundle all permissions you would need, and ask for them
before proceeding with the review.

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
  review '
This branch is an implementation of `crates/foo/SPEC.md`.
Check the branch diff and review the implementation.
Do not review or validate the spec itself, only it's implementation.
Do not run checks or tests, assume they all valid.
Do not look into other code or specs.
'
```

- Don't look into other code or specs
- Reject findings that are out of scope of the spec as not valuable. Don't let
  the implementation grow beyond the spec boundaries.
- Evaluate valuable findings: for each finding assign a severity: high, medium
  or low
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
