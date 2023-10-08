FROM ubuntu:18.04

RUN apt-get update -y
RUN apt-get install zlib1g-dev libbz2-dev libsnappy-dev git -y
RUN git clone https://github.com/facebook/rocksdb.git
RUN cd rocksdb; USE_RTTI=1 CFLAGS=-fPIC make static_lib; INSTALL_PATH=/usr make install make install; cd ..
