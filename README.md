Start server from testserver path:

python3 main.py


Write env var and start send puzzle message to server and read it back:

POOL_WS_ADDRESS=ws://localhost:8765 cargo run --release -- start --nodisplay --prover aaaa
