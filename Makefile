
all:
	dexy

ship: all
	git push origin
	s3deploy
