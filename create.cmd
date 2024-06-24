@echo off

copy NUL file
mkdir dir

mklink file-symlink-to-file file
mklink /d dir-symlink-to-file file
mklink file-symlink-to-dir dir
mklink /d dir-symlink-to-dir dir
mklink file-symlink-to-nonexistent nonexistent
mklink /d dir-symlink-to-nonexistent nonexistent

mklink file-symlink-to-file-symlink-to-file file-symlink-to-file
mklink /d dir-symlink-to-file-symlink-to-file file-symlink-to-file
mklink file-symlink-to-dir-symlink-to-file dir-symlink-to-file
mklink /d dir-symlink-to-dir-symlink-to-file dir-symlink-to-file
mklink file-symlink-to-file-symlink-to-dir file-symlink-to-dir
mklink /d dir-symlink-to-file-symlink-to-dir file-symlink-to-dir
mklink file-symlink-to-dir-symlink-to-dir dir-symlink-to-dir
mklink /d dir-symlink-to-dir-symlink-to-dir dir-symlink-to-dir
mklink file-symlink-to-file-symlink-to-nonexistent file-symlink-to-nonexistent
mklink /d dir-symlink-to-file-symlink-to-nonexistent file-symlink-to-nonexistent
mklink file-symlink-to-dir-symlink-to-nonexistent dir-symlink-to-nonexistent
mklink /d dir-symlink-to-dir-symlink-to-nonexistent dir-symlink-to-nonexistent

dir
