
all:
	dexy
	rsync --delete -azv index.rst output/* zedshaw.com:/var/www/zedshaw.com/music/
