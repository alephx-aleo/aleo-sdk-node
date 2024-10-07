# Aleo SDK Node

Aleo SDK Node allows program execution in NodeJS by interfacing with `snarkVM` under the hood.

Currently, it only supports:
1. Loading program keys for program execution
2. Build program execution transaction

For other use cases, please use [@provablehq/sdk](https://www.npmjs.com/package/@provablehq/sdk).

## Usage
To do program execution 

1. Install `aleo-sdk-node`
```bash
npm install aleo-sdk-node
```

2. Load program keys
Make sure you include `main.aleo` and `program.json` in the same directory. Then run `loadProgramKeys`.
```ts
await loadProgramKeys();
```

3. Build execution transaction
Use `buildExecutionTransaction` to build your transaction. Make sure you save the execution results for broadcasting to the network.
```ts
const privateKey = process.env.PRIVATE_KEY;
const options = {
    privateKey,
    endpoint: "https://api.explorer.provable.com/v1",
    functionName: "hello",
    inputs: [
        "5u32",
        "5u32"
    ],
    programId: "hello_hello.aleo",
    priorityFee: BigInt(0),
    baseFee: BigInt(11323),
    enableLog: true // NOTE: For debugging purposes
};
const execution = await buildExecutionTransaction(options);
```

4. Broadcast transaction to network
Using the execution transaction result, broadcast it to the network.
```ts
const txn = execution.transaction // NOTE: From output of step (3) build execution transaction
const url = new URL('https://api.explorer.provable.com/v1/testnet/transaction/broadcast');
const config = {
    body: txn,
    method: 'POST',
    headers: {
        "Content-Type": "application/json"
    }
}
const res = await fetch(url, config);
const data = await res.json();
```

## Network Support
For network, we currently only support testnet beta. Mainnet support is coming soon.

## Architecture Support
Currently, we only support the following architectures
- darwin-arm64
- darwin-x64
- linux-x64-musl

## Development
Package source code are in `/src`. To develop with the package, we can run the following command. It builds and runs `./test.mjs`. It is using `dotenv-cli`. Please install it globally (ie. `npm i -g dotenv-cli`).

```bash
# Build and run 
npm run dev
```
