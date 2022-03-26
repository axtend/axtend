# Node for Axtend networks

ARG DOCKER_IMAGE
ARG SHA
FROM "$DOCKER_IMAGE:$SHA"
USER moonbeam

COPY --chown=moonbeam build/moonbeam /moonbeam/moonbeam
RUN chmod uog+x /moonbeam/moonbeam
