src/
├─ main.rs
├─ grpc_clients.rs # quản lý các gRPC client (User, Order, Product...)
│
├─ routes/ # chỉ define REST endpoints
│ ├─ mod.rs
│ ├─ user.rs
│ ├─ order.rs
│ └─ orchestration.rs # routes cho use case tổng hợp
│
├─ handlers/ # map request -> orchestration
│ ├─ mod.rs
│ ├─ user_handler.rs
│ ├─ order_handler.rs
│ └─ orchestration_handler.rs
│
├─ services/ # business/orchestration logic
│ ├─ user_service.rs
│ ├─ order_service.rs
│ └─ orchestration/
│ ├─ user_order.rs
│ ├─ order_inventory.rs
│ └─ cart_product.rs
│
└─ utils.rs # logging, graceful shutdown, etc.
