# tar

# compress a folder
tar -cf archive.tar my_folder
tar -czf archive.tar.gz my_folder
tar -cJf archive.tar.xz my_folder

# compress a folder using relative file paths
tar -cf archive.tar -C my_folder .

# extract a folder with auto-detection of compression type
tar -xf archive.tar.gz

# extract into a specific folder
tar -xf archive.tar.gz -C my_folder

# extracing from stdin, explicit compression type specification required
# `z` - gzip, `j` - bzip, `J` - xz
curl my-data.com | tar -xJ

# list the archive files
tar -tf archive.tar.xz
