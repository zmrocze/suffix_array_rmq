
# SA + RMQ

For a given string $xs$ we are interested in longest common prefixes between some $xs[i..]$ and $xs[j..]$.
Here we give rust implementation of a structure that answers such queries in O(1) and takes O(n) to create.

## Develop

Run tests with

```
cargo test
```

### versions

Tested with rust `1.80.0-nightly`.