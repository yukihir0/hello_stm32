# hello_stm32

## Build

```
% cargo build
```

## Run on PC

```
% cargo run
```

## Run on Board

```
# terminal1
% openocd

# terminal2
% arm-none-eabi-gdb target/thumbv7em-none-eabi/debug/hello_stm32 -q -x openocd.gdb
```

## License

Copyright &copy; 2020 yukihir0
