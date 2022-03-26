sudo rm -rf /tmp/\*

# Generate a raw chain spec

/home/sankar/Zeeve/axia/target/release/axia \
build-spec \
--chain axia-staging \
--disable-default-bootnode --raw > /home/sankar/asr.json

### Telemetry core

/home/sankar/telemetry_core

### Telemetry shard

/home/sankar/telemetry_shard

### Telemetry frontend

cd /home/sankar/Cache/substrate-telemetry-exporter && yarn start
cd /home/sankar/Cache/substrate-telemetry/frontend && yarn start
cd /home/sankar/Cache/apps && yarn start --port 3005

### One

/home/sankar/Zeeve/axia/target/release/axia \
--chain /home/sankar/asr.json \
-d /tmp/chains/one \
--name axia_one \
--validator \
--port 30331 \
--ws-port 9941 \
--rpc-port 9931 \
--telemetry-url 'ws://localhost:8001/submit 0' \
--allow-private-ipv4 \
--node-key 0x74a8cfbadb5d2b0178ec124791bfa8346ac3550a4f689923c806428090055277

### Two

/home/sankar/Zeeve/axia/target/release/axia \
--chain /home/sankar/asr.json \
-d /tmp/chains/two \
--name axia_two \
--validator \
--port 30332 \
--ws-port 9942 \
--rpc-port 9932 \
--telemetry-url 'ws://localhost:8001/submit 0' \
--allow-private-ipv4 \
--bootnodes /ip4/127.0.0.1/tcp/30331/p2p/12D3KooWRm651Kd5GmsLTHJbgX5chQS5npx9ttLgo46UsegCMoNM

### Three

/home/sankar/Zeeve/axia/target/release/axia \
--chain /home/sankar/asr.json \
-d /tmp/chains/three \
--name axia_three \
--validator \
--port 30333 \
--ws-port 9943 \
--rpc-port 9933 \
--telemetry-url 'ws://localhost:8001/submit 0' \
--allow-private-ipv4 \
--bootnodes /ip4/127.0.0.1/tcp/30331/p2p/12D3KooWRm651Kd5GmsLTHJbgX5chQS5npx9ttLgo46UsegCMoNM

### Four

/home/sankar/Zeeve/axia/target/release/axia \
--chain /home/sankar/asr.json \
-d /tmp/chains/four \
--name axia_four \
--validator \
--port 30334 \
--ws-port 9944 \
--rpc-port 9934 \
--telemetry-url 'ws://localhost:8001/submit 0' \
--allow-private-ipv4 \
--bootnodes /ip4/127.0.0.1/tcp/30331/p2p/12D3KooWRm651Kd5GmsLTHJbgX5chQS5npx9ttLgo46UsegCMoNM

### Deploy keys

cd /home/sankar/Cache/bin/help && node deployKeys.js

### Sudo key

0x5b482eab1018eaee6293d3aaf15e4cc26fedd711b1ad4fe127adc11367ac3e9b

### Axtend run node

/home/sankar/Zeeve/axtend/target/release/axtend \
-d /tmp/chains/five \
--port 30335 \
--ws-port 9945 \
--rpc-port 9935 \
--dev \
-- --execution wasm \
--chain /home/sankar/asr.json \
--port 30336 \
--ws-port 9946 \
--rpc-port 9936

intact tuition pattern icon trophy typical pipe since merry notable cute brush

./axia-collator --collator --alice --force-authoring --tmp \
--port 30335 \
--ws-port 9945 \
--rpc-port 9935 -- --execution wasm --chain /home/sankar/asr.json \
--port 30336 \
--ws-port 9946 \
--rpc-port 9936
--telemetry-url "ws://localhost:8001/submit 0"

./axia-collator --collator --alice --force-authoring --tmp \
--port 30337 \
--ws-port 9947 \
--rpc-port 9937 -- --execution wasm --chain /home/sankar/asr.json \
--port 30338 \
--ws-port 9948 \
--rpc-port 9938
--telemetry-url "ws://localhost:8001/submit 0"

./axia-collator --tmp \
--port 30339 \
--ws-port 9949 \
--rpc-port 9939 \
-- --execution wasm --chain /home/sankar/asr.json \
--port 30340 \
--ws-port 9950 \
--rpc-port 9940

/home/sankar/Zeeve/axtend/target/release/axtend \
--tmp \
--name axtend_test \
--port 30334 \
--ws-port 9944 \
--rpc-port 9934 \
--chain axtend

/home/sankar/Zeeve/axtend/target/release/axtend \
--tmp \
--name axtend_test \
--port 30334 \
--ws-port 9944 \
--rpc-port 9934 \
--chain axtend-staging

/home/sankar/Zeeve/axtend/target/release/axtend \
--tmp \
--name axtend_test \
--port 30334 \
--ws-port 9944 \
--rpc-port 9934 \
--chain dev

/home/sankar/Zeeve/axtend/target/release/axtend \
-d /tmp/chains/five \
--port 30335 \
--ws-port 9945 \
--rpc-port 9935 \
--chain <chain_name:"axtend|axtend-dev|moonbase-dev"> \
-- --execution wasm \
--chain <relay_chain_spec> \
--port 30336 \
--ws-port 9946 \
--rpc-port 9936

/home/sankar/Zeeve/axtend/target/release/axtend \ -d /tmp/chains/five \ --port 30335 \ --ws-port 9945 \ --rpc-port 9935 \ --chain <chain_name:"axtend|axtend-dev|moonbase-dev"> \ -- --execution wasm \ --chain <relay_chain_spec> \ --port 30336 \ --ws-port 9946 \ --rpc-port 9936

/home/sankar/Zeeve/axtend/target/release/axtend \
--tmp \
--name axtend_test \
--port 30334 \
--ws-port 9944 \
--rpc-port 9934 \
--dev

#### Don't forget to put validator

/home/sankar/Zeeve/axtend/target/release/axtend \
--tmp \
--name axtend_test \
--port 30334 \
--ws-port 9944 \
--rpc-port 9934 \
--alice \
--chain axtend-dev

/home/sankar/Zeeve/axtend/target/release/axtend \
--tmp \
--name axtend_test \
--port 30334 \
--ws-port 9944 \
--rpc-port 9934 \
--alice \
--chain axtend-dev
