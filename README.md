# cosmos block stats substreams

### Generate protos
```bash
make protogen
```

### Build substreams
```bash
make build
```

### Run substreams
```bash
substreams run substreams.yaml block_to_stats -e mainnet.injective.streamingfast.io:443 -s 64987400 rx-t +1000
```
