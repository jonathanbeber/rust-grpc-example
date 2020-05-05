## Introduction

This document describes the changes in the system needed to implement a second user story, implemented on top of the `001_list_products_api.md` and the `002_list_products_client.md` ADR.

### Scope

This document describes the following [user story][0] implementation for the backend and frontend system:

- "As a consumer in store running the application
  I can chose to see unavailable products in this store
  So that I can subscribe for waiting list"

### Out Of Scope

This document does not describe any approach to implement the goal of the user story, "... so that I can subscribe for the waiting list". The concept of a user and a waiting list are out of scope and will be addressed in further documents.

## Stock changes

It'l' be needed to change the system so a new filter will allow users to set unavailable products. To achieve this goal a few system components will be changed.

### Messages

The stock request message will be changed, including a new boolean field, `display_unavailable_items`.

```protobuf
message StockRequest {
  Store store = 1;
  bool display_unavailable_items = 2;
}
```

The Item message will also include a new `availability_description` enum field, which, currently, has two possible values: `AVAILABLE` and `UNAVAILABLE`. For now, this field will be simply assigned with the logic that if a store contains 1 or more of a specific product it's `AVAILABLE`, otherwise, it's `UNAVAILABLE`.

```protobuf
enum AvailabilityDescription {
  UNAVAILABLE = 0;
  AVAILABLE = 1;
}

message Item {
  string Name = 1;
  string Brand = 2;
  string Category = 3;
  int32 Quantity = 4;
  AvailabilityDescription availability_description = 5;
}
```

### Commands

A new option will be added to the `list` subcommand of the CLI application. It'll be a boolean option `--show-unavailable` and its value will affect directly the `StockRequest` message to be issued by the frontend.

### CLI Output

The CLI output table will have a new column added, responsible for describing the `availability_description` field added on StockResponse. This column will be named "STATUS".
