CREATE TABLE IF NOT EXISTS balance_changes (
   `contract` TEXT NOT NULL,
   `owner` TEXT NOT NULL,
   `amount` INT NOT NULL,
   `old_balance` INT NOT NULL,
   `new_balance` INT NOT NULL,
   `transaction_id` TEXT NOT NULL,
   `block_num` INT NOT NULL,
   `timestamp` TEXT NOT NULL,
   `change_type` INT NOT NULL,
);