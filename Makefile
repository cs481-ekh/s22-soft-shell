projdir := st_interpret

build:
	cd $(projdir) && ./build.sh
test:
	cd $(projdir) && ./test.sh

clean:
	cd $(projdir) && ./clean.sh

