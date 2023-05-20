use clap::Parser;

#[derive(Parser)]
struct CliArgs {
    #[clap(name = "OP_RETURN msg", short = 'm', long = "msg", required = true)]
    opreturn_msg: String,
    #[clap(name = "amount in sat", short = 'a', long = "amount", required = true)]
    amount: u64,
    #[clap(name = "outpoint txid", short = 't', long = "txid", required = true)]
    txid: bitcoin::Txid,
    #[clap(name = "outpoint vout", short = 'v', long = "vout", required = true)]
    vout: u32,
}

fn main() {
    let args = CliArgs::parse();
    let out_script = bitcoin::blockdata::script::Builder::new()
        .push_opcode(bitcoin::blockdata::opcodes::all::OP_RETURN)
        .push_slice::<&bitcoin::blockdata::script::PushBytes>(
            args.opreturn_msg.as_bytes().try_into().unwrap(),
        )
        .into_script();
    let tx = bitcoin::Transaction {
        version: 1,
        lock_time: bitcoin::absolute::LockTime::from_height(0).unwrap(),
        input: vec![bitcoin::TxIn {
            previous_output: bitcoin::OutPoint::new(args.txid, args.vout),
            script_sig: Default::default(),
            sequence: bitcoin::blockdata::transaction::Sequence::MAX,
            witness: bitcoin::blockdata::witness::Witness::new(),
        }],
        output: vec![bitcoin::TxOut {
            script_pubkey: out_script,
            value: args.amount,
        }],
    };
    println!("{}", bitcoin::consensus::encode::serialize_hex(&tx));
}
