FROM rust:latest

RUN apt update -y && \
	apt install -y nodejs npm && \
	cargo install wasm-pack && \
	npm install npm@latest -g

# RUN npm init wasm-app /home/rusgb/app
EXPOSE 8080
WORKDIR /home/rusgb

CMD ["/bin/bash"]
