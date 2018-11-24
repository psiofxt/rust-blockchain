# rust-blockchain
An implementation of py-blockchain in rust!

In this project is ~40% ported from the python project. The registering of nodes
and resolving conflicts with the longer chain remain to be done

# Running
Simply clone the repo and `cargo run`.

Mine a few blocks with `localhost:8080/mine` then go to `localhost:8080/chain` to
see the entire chain and block reward coinbase payouts. 
