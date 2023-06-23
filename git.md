Git
===

Remove the last commit:
```sh
git reset HEAD~1
git push --force
```

Checkout Github pull request:
```sh
gh pr checkout pull-request
```

Squash merge
```sh
git merge --squash bugfix
git commit
```

Add remote:
```sh
git remote add site user@host:path
```

Allow push to not bare master:
```sh
git config receive.denyCurrentBranch updateInstead
```

Add a tag to commit:
```sh
git tag -a v1.0.0 -m "Some description"
```

Delete a tag:
```sh
git tag -d v1.0.0
git push --delete origin v1.0.0
```

Merge only specific files from another branch:

```sh
git checkout source_branch <paths>...
```

Remove all branches except of master and the current one:

```sh
git branch | grep -v "master" | xargs git branch -D
```


Shallow clone: `git clone --depth 1 repo`


Deployment with `post-receive` hook:

```sh
cat > .git/hooks/post-receive << EOF
#!/usr/bin/env bash
while read OLD NEW REF; do
    if [ \$REF = "refs/heads/master" ]; then
        if git diff --name-only \$OLD \$NEW | grep -q 'requirements.txt'; then
            echo 'Installing python requirements ...'
            ../var/env/bin/pip install -qU pip wheel -r ../requirements.txt
        fi
    fi
done
EOF
chmod +x .git/hooks/post-receive
```
