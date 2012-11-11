
all:
	dexy
	cp -r stylesheets javascripts output/

sync: all
	rsync --delete -azv index.rst output/* orkestrix.org:/var/www/orkestrix.org/
