# собрать всё

cargo build

# собрать только клиентскую библиотеку

cargo build -p client

# собрать только imitator

cargo build -p imitator

cargo run -p client-- 127.0.0.1:7890
cargo run -p client-- 127.0.0.1:7890
