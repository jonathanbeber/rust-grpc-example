## Introduction

Check the previous ADR `001_list_products_api.md` for more details.

### Scope

This document describes the first version of the stock service client, containing only the client able to request for products on a defined store. It approaches the client service for the first user story, mentioned on the previous document.

### Out Of Scope

This document does not describe the backend service (for this, check the previously ADR `001_list_products_api.md`).

## Stock CLI

The service defined here will be called "stock CLI". It consists of a CLI tool responsible for querying the "stock service", defined on the ADR `001_list_products_api.md`, here mentioned as "backend".

### Protocol

This service will be responsible for calling the backend service. This way it has to respect the defined protocol resulted from the implementation of the first ADR. Once again, this document defines [protobuf][0] specs, but not protocol-specific details.

### Messages

This service will use the same messages defined on the first ADR.

### Available commands

On this moment, only one command is available:

- `list`: This command query the backend service and show to the user information about the available products. The following parameters are accepted:
  - `--store|-s`: defines the store to be queried. Currently, the possible values are `BERLIN_DE` or `VENEZA_IT` and are defined on the first ADR as a proto
  definition, shared with this client application.

An extra parameter `--addr` is also accepted to inform the client of the host and port the backend service is running.


[0]: https://developers.google.com/protocol-buffers
