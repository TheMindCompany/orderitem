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

In an effort to make configuration of `orderitem` simple we have defined a YAML config to handle credentials if not already configured.  Credentials will first look for environment settings on system and fall back to this `config.yaml` file when not found.

The first time you run the cli utility you will be prompted to configure existing credentials are not found when not setting environment fields.  

#### Environment

This configuration can managed with environment values.

```
ORDERITEM_HOST=127.0.0.1
ORDERITEM_PORT=8080
ORDERITEM_CONFIG_BASE=/home/user/.orderitem
ORDERITEM_SQL_HOST=127.0.0.1
ORDERITEM_SQL_PORT=3306
ORDERITEM_SQL_USERNAME=root
ORDERITEM_SQL_PASSWORD=pass
```

#### YAML

This configuration can managed through `~/.orderitem/config.yaml`.  To change the base path you can set it with an environment field `CONFIG_orderitem_BASE`.

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

**Seed new database with table and sample data.**  
```bash
orderitem --seed
```

**Creaete a new order item.**  
```bash
orderitem create --customer-id 10 --sku-id item-01-a --upload-id 59
```

**Read a order item.**  
```bash
orderitem read --order-id 1
```

**Update a order item status.**  
```bash
orderitem update --order-id 1 --status DONE
```

**Delete a order item.**  
```bash
orderitem delete --order-id 1
```

### Using as Daemon

Daemon mode runs OrderITEM as HTTP REST service.  You can either use the options arguments to predefine route values or environment value equivalents. See below examples.

---
#### Run Daemon In Container

```bash
orderitem --host 0.0.0.0 --port 8080 --daemon
```

#### Daemon HTTP Routes
##### Field Definitions

Field definitions can be found in every response containing order information.  The same fields are used in updating orders.

**order_id** INT  
The reference ID to an order item and all associated  activity.

**status** String  
Current status of the order.

**customer_id** INT  
Customer ID associated to order item.

**payment_id** INT  
Payment ID associated to the item purchase.

**shipping_id** INT  
Shipping ID associated to item cargo.

**upload_id** INT  
Upload ID for printing of sku.

**sku_id** String  
Stock unit identifier to be used in completing order item.

**quantity**  INT  
Number of item to be ordered.

**discount** String  
Discount assignment code.

**ready_to_ship** bool  
Is the order packed and labeled for shipping pickup marker.

**shipped_on** String  
Date order item was shipped

**notes** String
Notes to assist in order handling.

**created_on** String
When the order item was created.

---

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

Failure:  
**STATUS 400**

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

Failure:  
**STATUS 400**

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

Failure:  
**STATUS 204**

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

Failure:  
**STATUS 400**

##### DELETE /delete/{order-id}

Delete an order.  This changes an order's status to "CANCELED".

Response:  
**STATUS 202**

Failure:  
**STATUS 400**

---

## Work In Progress
Feel free to contribute or use in any way.

Built with Rust love <3
