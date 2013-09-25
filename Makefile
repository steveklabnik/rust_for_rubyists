CHAPTERS=book/preamble.md \
				 book/chapter-01.html \
				 book/chapter-02.html \
				 book/chapter-03.html \
				 book/chapter-04.html \
				 book/chapter-05.html \
				 book/chapter-06.html \
				 book/chapter-07.html \
				 book/chapter-08.html \
				 book/chapter-09.html \
				 book/chapter-10.html \
				 book/chapter-11.html \
				 book/chapter-12.html

all: rust-for-rubyists.epub site

rust-for-rubyists.epub: $(CHAPTERS) book/metadata.xml book/title.txt
	pandoc --toc -S -s --epub-metadata=book/metadata.xml -o rust-for-rubyists.epub book/title.txt book/preamble.md book/chapter-*.md

book/book.html: $(CHAPTERS)
	pandoc -S -s -o book/book.html --include-before-body=book/header.html --include-after-body=book/footer.html $(CHAPTERS)

book/%.html : book/%.md
		pandoc -S -s -o $@ --include-before-body=book/header.html --include-after-body=book/footer.html $<

site: $(CHAPTERS) book/book.html

ship: all
	git push origin
	s3deploy
