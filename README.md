# network_zenoh

`CITA-Cloud`中[network微服务](https://github.com/cita-cloud/cita_cloud_proto/blob/master/protos/network.proto)的实现，基于[zenoh](https://crates.io/crates/zenoh)。

## 编译docker镜像
```
docker build -t citacloud/network_zenoh .
```

## 使用方法

```
$ network -h
network 6.5.0
Rivtower Technologies <contact@rivtower.com>
This doc string acts as a help message when the user runs '--help' as do all doc strings on fields

USAGE:
    network <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    run     run this service

```

### network-run

运行`network`服务。

```
$ network run -h
network-run 
run this service

USAGE:
    network run [OPTIONS]

OPTIONS:
    -c, --config <CONFIG_PATH>    Chain config path [default: config.toml]
    -h, --help                    Print help information
    -l, --log <LOG_FILE>          log config path [default: network-log4rs.yaml]

```

参数：
1. `config` 微服务配置文件。

    参见示例`example/config.toml`。

    其中：
    * `ca_cert` 为`CA`根证书。
    * `cert` 为节点证书。
    * `priv_key` 为节点证书对应的私钥。
    * `grpc_port` 为`gRPC`服务监听的端口号。
    * `listen_port` 为节点网络的监听端口。
    * `peers` 为邻居节点的网络信息，其中`host`字段为`ip`或者域名，`port`字段为端口号，`domain`字段为该邻居节点申请证书时使用的域名。
    * `reconnect_timeout` 当无法网络连接到某个邻居节点时，尝试重连的超时时间。
    * `try_hot_update_interval` 为配置文件热更新功能，扫描间隔，单位为秒。

2. 日志配置文件。

    参见示例`network-log4rs.yaml`。

    其中：

    * `level` 为日志等级。可选项有：`Error`，`Warn`，`Info`，`Debug`，`Trace`，默认为`Info`。
    * `appenders` 为输出选项，类型为一个数组。可选项有：标准输出(`stdout`)和滚动的日志文件（`journey-service`），默认为同时输出到两个地方。


```
$ network run -c example/config.toml -l network-log4rs.yaml
```

## 设计

