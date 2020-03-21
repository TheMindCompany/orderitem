# OrderITEM

Create, Read, Update, Delete order items.  Use as a cli based utility for daily engineering or as HTTP REST service.

## Install

Use the below command to install binary or build from source.

Binary install:  

```bash
curl https://themindcompany.github.io/orderitem/install.sh -sS | bash -s
```

Source install:

```bash
git clone https://github.com/TheMindCompany/orderitem.git
cd orderitem
make build
make install
```

### Configuration

In an effort to make configuration of `orderitem` simple we have defined a YAML config to handle credentials if not already configured.  Credentials will first look for service credentials on system and fall back to this `config.yaml` file when not found. This config can managed through `~/.orderitem/config.yaml`.  To change the base path you can set it with an environment field `CONFIG_orderitem_BASE`.

The first time you run the cli utility you will be prompted to configure existing credentials are not found.  

**~/.orderitem/config.yaml**  
```yaml
kind: config
version: alpha/1.0
specs:
  sql:
    host: myhost.com
    port: 3306
    user: ***
    pass: ***
```

### Autocompletion

For convenience purposes autocompletion scripts have been provided for most major shell programs.  Hopefully this make it more useable for daily use if engineers.

More information for each completion script provided:

```bash
orderitem configuration --help
```

## USAGE

Refer to the help menu for details `-h` or `--help`.

```bash
USAGE:
    orderitem [FLAGS] [OPTIONS] [method] [SUBCOMMAND]

FLAGS:
        --no-buckets    Don't allow bucket to change
    -g, --gen-key       Generate key's with UUIDv4
    -d, --daemon        Daemon mode
    -v, --verbose       Enable verbose logging
    -h, --help          Prints help information

OPTIONS:
    -b, --bucket <bucket>      Bucket target for signature
    -k, --key <key>            Key path target. (ie: filename)
        --prefix <prefix>      Let util append filename to key prefix
    -r, --region <region>      Region target [env: AWS_DEFAULT_REGION=]  [default: us-east-1]
    -t, --timeout <timeout>    Duration URL is invalid in milliseconds [default: 60000]
        --port <port>          Daemon mode port [env: orderitem_PORT=]  [default: 8080]
        --host <host>          Daemon mode host [env: orderitem_HOST=]  [default: 127.0.0.1]

ARGS:
    <method>    The type of method being requested for signing url [default: PUT]

SUBCOMMANDS:
    configuration    Configuration options
    help             Prints this message or the help of the given subcommand(s)
```

### Using the CLI

```bash
orderitem PUT --bucket my-bucket --key path/to/file.txt
```

```bash
orderitem GET --bucket my-bucket --key path/to/file.txt
```

```bash
orderitem DEL --bucket my-bucket --key path/to/file.txt
```

### Using as Daemon

Daemon mode runs orderitem as HTTP REST service.  You can either use the options arguments to predefine route values or environment value equivalents. See below examples.

---

#### Fully Customized Signed Url's Allowed

```bash
orderitem --host 0.0.0.0 --port 8080 --daemon
```

##### GET /create/{bucket}/{key-path:*}

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "PUT",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```

##### GET /read/{bucket}/{key-path:*}

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "GET",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```

##### GET /delete/{bucket}/{key-path:*}

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "DELETE",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```

---

#### Strongly Defined Signed Url's Only

```bash
orderitem --bucket my-bucket --no-buckets --prefix user/bin --gen-key --daemon
```

##### GET /create

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "PUT",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```

##### GET /read/{key-path:*}

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "GET",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```

##### GET /delete/{key-path:*}

Response:  
```json
{
  "data": {
    "attributes": {
      "url": "<signed-url>",
      "method": "DELETE",
      "ttl": 60000,
      "engine": "aws",
      "request": "<uri-received-for-request>"
    }
  }
}
```
___

Find more ways to configure it using `--help` flag. The above two use case examples show routes accessible when deployed with different parameters.


## Benchmark

Benchmark was made using most difficult route that with bottom level exceptions and generating UUIDv4 hash on localhost.

**Test System Spec**
- Macbook Pro
- 2.6 GHz 6-Core Intel Core i7
- 16 GB 2400 MHz DDR4
- Intel UHD Graphics 630 1536 MB

```bash
ab -n 10000 -c 50 http://127.0.0.1:8080/create
```

```
Server Software:
Server Hostname:        127.0.0.1
Server Port:            8080

Document Path:          /create
Document Length:        427 bytes

Concurrency Level:      50
Time taken for tests:   1.719 seconds
Complete requests:      10000
Failed requests:        0
Total transferred:      5360000 bytes
HTML transferred:       4270000 bytes
Requests per second:    5817.97 [#/sec] (mean)
Time per request:       8.594 [ms] (mean)
Time per request:       0.172 [ms] (mean, across all concurrent requests)
Transfer rate:          3045.34 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    1   0.6      1       4
Processing:     1    7  12.4      3     124
Waiting:        0    7  12.4      2     124
Total:          1    9  12.3      4     125

Percentage of the requests served within a certain time (ms)
  50%      4
  66%      5
  75%      5
  80%      7
  90%     24
  95%     31
  98%     43
  99%     67
 100%    125 (longest request)
 ```

---

## Work In Progress
Feel free to contribute or use in any way.

Built with Rust love <3
