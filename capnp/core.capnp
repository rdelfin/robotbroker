@0xebff5c8d935187a8;

struct CreateNodeRequest {
    name @0 :Text;
}

struct DeleteNodeRequest {
    name @0 :Text;
}

interface Node {
    createNode @0 (req :CreateNodeRequest) -> ();
    deleteNode @1 (req: DeleteNodeRequest) -> ();
}
