import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type AddTransactionsResponse = { 'ok' : Stats } |
  { 'err' : string } |
  { 'Full' : Stats };
export interface Archive {
  'append_transactions' : ActorMethod<
    [Array<Transaction>],
    AddTransactionsResponse
  >,
  'cycles' : ActorMethod<[], bigint>,
  'deposit_cycles' : ActorMethod<[], undefined>,
  'get_transaction' : ActorMethod<[TxIndex], [] | [Transaction]>,
  'icrc3_get_blocks' : ActorMethod<
    [Array<TransactionRange>],
    GetTransactionsResult
  >,
  'remaining_capacity' : ActorMethod<[], bigint>,
  'total_transactions' : ActorMethod<[], bigint>,
}
export interface ArchiveInitArgs {
  'indexType' : IndexType,
  'maxPages' : bigint,
  'maxRecords' : bigint,
  'firstIndex' : bigint,
}
export interface ArchivedTransactionResponse {
  'args' : Array<TransactionRange>,
  'callback' : GetTransactionsFn,
}
export type GetTransactionsFn = ActorMethod<
  [Array<TransactionRange>],
  GetTransactionsResult
>;
export interface GetTransactionsResult {
  'log_length' : bigint,
  'blocks' : Array<{ 'id' : bigint, 'block' : Value }>,
  'archived_blocks' : Array<ArchivedTransactionResponse>,
}
export type IndexType = { 'Stable' : null } |
  { 'StableTyped' : null } |
  { 'Managed' : null };
export interface Stats {
  'region' : { 'id' : bigint, 'size' : bigint },
  'currentPages' : bigint,
  'memory' : { 'pages' : [] | [bigint], 'type_of' : IndexType },
  'currentOffset' : bigint,
  'itemCount' : bigint,
  'maxPages' : bigint,
}
export type Transaction = { 'Int' : bigint } |
  { 'Map' : Array<[string, Value]> } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string } |
  { 'Array' : Array<Value> };
export interface TransactionRange { 'start' : bigint, 'length' : bigint }
export type TxIndex = bigint;
export type Value = { 'Int' : bigint } |
  { 'Map' : Array<[string, Value]> } |
  { 'Nat' : bigint } |
  { 'Blob' : Uint8Array | number[] } |
  { 'Text' : string } |
  { 'Array' : Array<Value> };
export interface _SERVICE extends Archive {}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
