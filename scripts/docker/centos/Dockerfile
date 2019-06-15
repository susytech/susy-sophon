FROM centos:latest

RUN mkdir -p /opt/susy/data && \
    chmod g+rwX /opt/susy/data && \
    mkdir -p /opt/susy/release

COPY susy/susy /opt/susy/release

WORKDIR /opt/susy/data

# exposing default ports
#
#           secret
#      ipfs store     ui   rpc  ws   listener  discovery
#      ↓    ↓         ↓    ↓    ↓    ↓         ↓
EXPOSE 5001 8082 8083 8180 8545 8546 30303/tcp 30303/udp

# switch to non-root user
USER 1001

#if no base path provided, assume it's current workdir
CMD ["--base-path","."]
ENTRYPOINT ["/opt/susy/release/susy"]  


   


