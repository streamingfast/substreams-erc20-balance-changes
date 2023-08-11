create table if not exists balance_changes {
    id text not null constraint balance_changes_pk primary key,
    contract text not null,
    owner text not null,
    amount int not null,
    old_balance int not null,
    new_balance int not null,
    transaction_id text not null,
    block_num int not null,
    timestamp text not null,
}