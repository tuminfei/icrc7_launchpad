export const idlFactory = ({ IDL }) => {
  const GetTransactionsResult = IDL.Rec();
  const Value = IDL.Rec();
  const IndexType = IDL.Variant({
    'Stable' : IDL.Null,
    'StableTyped' : IDL.Null,
    'Managed' : IDL.Null,
  });
  const ArchiveInitArgs = IDL.Record({
    'indexType' : IndexType,
    'maxPages' : IDL.Nat,
    'maxRecords' : IDL.Nat,
    'firstIndex' : IDL.Nat,
  });
  Value.fill(
    IDL.Variant({
      'Int' : IDL.Int,
      'Map' : IDL.Vec(IDL.Tuple(IDL.Text, Value)),
      'Nat' : IDL.Nat,
      'Blob' : IDL.Vec(IDL.Nat8),
      'Text' : IDL.Text,
      'Array' : IDL.Vec(Value),
    })
  );
  const Transaction = IDL.Variant({
    'Int' : IDL.Int,
    'Map' : IDL.Vec(IDL.Tuple(IDL.Text, Value)),
    'Nat' : IDL.Nat,
    'Blob' : IDL.Vec(IDL.Nat8),
    'Text' : IDL.Text,
    'Array' : IDL.Vec(Value),
  });
  const Stats = IDL.Record({
    'region' : IDL.Record({ 'id' : IDL.Nat, 'size' : IDL.Nat64 }),
    'currentPages' : IDL.Nat64,
    'memory' : IDL.Record({
      'pages' : IDL.Opt(IDL.Nat64),
      'type_of' : IndexType,
    }),
    'currentOffset' : IDL.Nat64,
    'itemCount' : IDL.Nat,
    'maxPages' : IDL.Nat64,
  });
  const AddTransactionsResponse = IDL.Variant({
    'ok' : Stats,
    'err' : IDL.Text,
    'Full' : Stats,
  });
  const TxIndex = IDL.Nat;
  const TransactionRange = IDL.Record({
    'start' : IDL.Nat,
    'length' : IDL.Nat,
  });
  const GetTransactionsFn = IDL.Func(
      [IDL.Vec(TransactionRange)],
      [GetTransactionsResult],
      ['query'],
    );
  const ArchivedTransactionResponse = IDL.Record({
    'args' : IDL.Vec(TransactionRange),
    'callback' : GetTransactionsFn,
  });
  GetTransactionsResult.fill(
    IDL.Record({
      'log_length' : IDL.Nat,
      'blocks' : IDL.Vec(IDL.Record({ 'id' : IDL.Nat, 'block' : Value })),
      'archived_blocks' : IDL.Vec(ArchivedTransactionResponse),
    })
  );
  const Archive = IDL.Service({
    'append_transactions' : IDL.Func(
        [IDL.Vec(Transaction)],
        [AddTransactionsResponse],
        [],
      ),
    'cycles' : IDL.Func([], [IDL.Nat], ['query']),
    'deposit_cycles' : IDL.Func([], [], []),
    'get_transaction' : IDL.Func([TxIndex], [IDL.Opt(Transaction)], ['query']),
    'icrc3_get_blocks' : IDL.Func(
        [IDL.Vec(TransactionRange)],
        [GetTransactionsResult],
        ['query'],
      ),
    'remaining_capacity' : IDL.Func([], [IDL.Nat], ['query']),
    'total_transactions' : IDL.Func([], [IDL.Nat], ['query']),
  });
  return Archive;
};
export const init = ({ IDL }) => {
  const IndexType = IDL.Variant({
    'Stable' : IDL.Null,
    'StableTyped' : IDL.Null,
    'Managed' : IDL.Null,
  });
  const ArchiveInitArgs = IDL.Record({
    'indexType' : IndexType,
    'maxPages' : IDL.Nat,
    'maxRecords' : IDL.Nat,
    'firstIndex' : IDL.Nat,
  });
  return [ArchiveInitArgs];
};
