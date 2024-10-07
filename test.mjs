import { buildExecutionTransaction, loadProgramKeys } from './index.js';

await loadProgramKeys();
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
    enableLog: true
};
const broadcast = async (txn) => {
    const url = new URL('https://api.explorer.provable.com/v1/testnet/transaction/broadcast');
    const config = {
        body: txn,
        method: 'POST',
        headers: {
            "Content-Type": "application/json"
        }
    }
    const res = await fetch(url, config);
    console.debug('Response status:', res.status);
    const data = await res.json();

    return data;
};

const execution = await buildExecutionTransaction(options);
const result = await broadcast(execution.transaction);
console.log('[DONE] Broadcast result:', result);
