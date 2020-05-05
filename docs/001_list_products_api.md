## Introduction

This ADR consists of the description of a new service, responsible for listing to clients the available products in a given store.

### Scope

This document describes the first version of the service, containing only the API that lists products available in a store. It approaches the backend service for the following user story]:

- "As a consumer in store running the application
  I can view a list of products in this store (e.g. berlin-de)
  So that I know which products are currently available to rent"

### Out Of Scope
This document does not describe the front end service or CLI, neither try to include other features. These topics will be addressed in further documents.

## Stock service
The service defined here will be called "stock service". It consists of a backend service to be queried by the customer-facing application, here called "front-end".

### Protocol
It has to be accessible from the front-end with a protocol such as HTTP, gRPC, GraphQL, etc. The decision of the protocol is beyond this document scope and has to be aligned with the stack arrangement, such as language, runtime, network infrastructure, etc., therefore, this document defines [protobuf][0] specs, but not protocol-specific details.

### Messages

The stock service receives only one message in this first version defined by this document:

```protobuf
syntax = "proto3";

enum Store {
  BERLIN_DE = 0;
  VENEZA_IT = 1;
}

message StockRequest {
  Store store = 1;
}
```

It contains only one field `Store` responsible for defining the store stock to be queried, setting the 0 (a.k.a. default) value to `BERLIN_DE`.

The service returns the following possible messages:

#### Successful response

```protobuf
message Item {
  string Name = 1;
  string Brand = 2;
  string Category = 3;
  int32 Quantity = 4;
}

message StockResponse {
  repeated Item items = 1;
}
```

#### Non-expected (internal) errors

```protobuf
message ErrorResponse {
  enum Type {
    INTERNAL_SERVER_ERROR = 0;
  }
  Type type = 1;
  string domain = 2;
  string description = 3;
}
```

  The `type` field includes details for the returned error, at this moment the only possible value is `INTERNAL_SERVER_ERROR`.
  The domain field describes the service where the error occurred, predicting distributed systems. At this moment the possible value is `stock-service.example.com`.
  The description field is an open string field with more details from the error if needed. In the case of `INTERNAL_SERVER_ERROR`, the description is well limited to "The server encountered an internal error or misconfiguration and was unable to complete your request. Please contact the service administrator or try again."

The errors have inspiration on [Google APIs error protobuf specs][1].

### Data Source

For reasons of simplicity, this service does not include at the moment any external database. The list of available products and their `Availability` details is stored as a static file with the application source code.

[0]: https://developers.google.com/protocol-buffers
[1]: https://github.com/googleapis/googleapis/blob/master/google/rpc/error_details.proto
