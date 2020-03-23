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

```

### Using the CLI

**Seed new database with table**  
```bash
orderitem --seed
```


### Using as Daemon

Daemon mode runs OrderITEM as HTTP REST service.  You can either use the options arguments to predefine route values or environment value equivalents. See below examples.

---

#### Fully Customized Signed Url's Allowed

```bash
orderitem --host 0.0.0.0 --port 8080 --daemon
```

##### POST /create

Make a new order with a custom, sku and upload id.

Response:  
**STATUS 201 Content-Type: application/json**
```json
{
  "data": {
    "attributes": {
      "order_id": "",
      "customer_id": "",
      "upload_id": "",
      "sku_id": ""
    }
  }
}
```

##### POST /create/from/{order-id}

Make a new order from an existing open or closed order.  Precaution, this will close previous order if open.

Response:  
**STATUS 201 Content-Type: application/json**
```json
{
  "data": {
    "attributes": {
      "order_id": "",
      "customer_id": "",
      "upload_id": ""
    }
  }
}
```

##### GET /read/{order-id}

Get details on an order.

Response:  
**STATUS 200 Content-Type: application/json**
```json
{
  "data": {
    "attributes": {
      "order_id": "",
      "customer_id": "",
      "upload_id": ""
    }
  }
}
```

##### PUT /update/{order-id}

Update order fields.

Request:
```json
{
  "data": {
    "attributes": {
      "upload_id": ""
    }
  }
}
```

Response:  
**STATUS 202 Content-Type: application/json**
```json
{
  "data": {
    "attributes": {
      "order_id": "",
      "customer_id": "",
      "upload_id": ""
    }
  }
}
```

##### DELETE /delete/{order-id}

Delete an order.  This changes an order's status to "DELETE".

Response:  
**STATUS 202**


---

## Work In Progress
Feel free to contribute or use in any way.

Built with Rust love <3
