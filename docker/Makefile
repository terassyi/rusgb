NAME=rusgb
VERSION=latest
CWD=`pwd`

build:
	docker build -t $(NAME):$(VERSION) .

run:
	docker run -it --rm \
		-p 8080:8080 \
        -v $(CWD)/../src:/home/rusgb/src \
        -v $(CWD)/../tests:/home/rusgb/tests \
        -v $(CWD)/../Cargo.toml:/home/rusgb/Cargo.toml \
        -v $(CWD)/../Makefile:/home/rusgb/Makefile \
        -v $(CWD)/../app:/home/rusgb/app \
        -v $(CWD)/../scripts:/home/rusgb/scripts \
		--name $(NAME) \
		$(NAME):$(VERSION) bash

exec:
	docker exec -it $(NAME) bash

stop:
	docker rm -f $(NAME)

contener=`docker ps -a -q`
image=`docker images | awk '/^<none>/ { print $$3 }'`

clean:
	@if [ "$(image)" != "" ] ; then \
		docker rmi $(image); \
		fi
	@if [ "$(contener)" != "" ] ; then \
		docker rm $(contener); \
		fi
