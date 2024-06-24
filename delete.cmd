@echo off

del file
rmdir dir

del file-symlink-to-file
rmdir dir-symlink-to-file
del file-symlink-to-dir
rmdir dir-symlink-to-dir
del file-symlink-to-nonexistent
rmdir dir-symlink-to-nonexistent

del file-symlink-to-file-symlink-to-file
rmdir dir-symlink-to-file-symlink-to-file
del file-symlink-to-dir-symlink-to-file
rmdir dir-symlink-to-dir-symlink-to-file
del file-symlink-to-file-symlink-to-dir
rmdir dir-symlink-to-file-symlink-to-dir
del file-symlink-to-dir-symlink-to-dir
rmdir dir-symlink-to-dir-symlink-to-dir
del file-symlink-to-file-symlink-to-nonexistent
rmdir dir-symlink-to-file-symlink-to-nonexistent
del file-symlink-to-dir-symlink-to-nonexistent
rmdir dir-symlink-to-dir-symlink-to-nonexistent

dir
