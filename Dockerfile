FROM ubuntu

LABEL authors="Margarita Volodina,Roman Dronov"

RUN apt-get update
RUN apt-get install -y --force-yes build-essential git curl
RUN apt-get clean
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN cd /home/ && git clone https://github.com/margarita-v/optimization_methods_development.git

ENV HOME=/root
ENV PATH=${HOME}/.cargo/bin:${PATH}
