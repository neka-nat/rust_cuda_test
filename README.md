# rust_cuda_test

## Build

```sh
docker build -t rust-cuda .
```

```sh
docker run -it --gpus all -v $PWD:/root/rust-cuda --entrypoint /bin/bash rust-cuda
cargo run
```
