@echo off
mkdir directory
copy NUL directory\.keep
mklink file-symlink directory
mklink /d dir-symlink directory
dir
