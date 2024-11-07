CREATE TABLE IF NOT EXISTS balance_changes (
   "contract" TEXT NOT NULL,
   "owner" TEXT NOT NULL,
   "amount" NUMERIC NOT NULL,
   "old_balance" NUMERIC NOT NULL,
   "new_balance" NUMERIC NOT NULL,
   "transaction_id" TEXT NOT NULL,
   "block_num" INT NOT NULL,
   "timestamp" TEXT NOT NULL,
   "change_type" INT NOT NULL,
   "call_index" INT NOT NULL,
   PRIMARY KEY ("block_num", "transaction_id", "call_index")
);