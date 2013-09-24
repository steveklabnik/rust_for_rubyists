all: rust-for-rubyists.epub site

rust-for-rubyists.epub:
	pandoc --toc -S -s --epub-metadata=book/metadata.xml -o rust-for-rubyists.epub book/title.txt book/preamble.md book/chapter-*.md

site:
	pandoc -S -s book/preamble.md book/chapter-*.md -o book/book.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-01.md -o book/chapter-01.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-02.md -o book/chapter-02.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-03.md -o book/chapter-03.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-04.md -o book/chapter-04.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-05.md -o book/chapter-05.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-06.md -o book/chapter-06.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-07.md -o book/chapter-07.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-08.md -o book/chapter-08.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-09.md -o book/chapter-09.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-10.md -o book/chapter-10.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-11.md -o book/chapter-11.html --include-before-body=book/header.html --include-after-body=book/footer.html
	pandoc -S -s book/chapter-12.md -o book/chapter-12.html --include-before-body=book/header.html --include-after-body=book/footer.html

ship: all
	git push origin
	s3deploy
