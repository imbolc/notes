# Replace text in multiple files

git grep -l 'old' | xargs sed -i 's/old/new/g'
