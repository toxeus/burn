use bitcoin::hashes::hex::FromHex;
use structopt::StructOpt;

#[derive(StructOpt)]
struct CliArgs {
    #[structopt(name = "OP_RETURN msg", short = "m", long = "msg", required = true)]
    opreturn_msg: String,
    #[structopt(name = "amount in sat", short = "a", long = "amount", required = true)]
    amount: u64,
    #[structopt(
        name = "outpoint txid",
        short = "t",
        long = "txid",
        parse(try_from_str = bitcoin::Txid::from_hex),
        required = true,
    )]
    txid: bitcoin::Txid,
    #[structopt(name = "outpoint vout", short = "v", long = "vout", required = true)]
    vout: u32,
}

fn main() {
    let args = CliArgs::from_args();
    let out_script = bitcoin::blockdata::script::Builder::new()
        .push_opcode(bitcoin::blockdata::opcodes::all::OP_RETURN)
        .push_slice(args.opreturn_msg.as_bytes())
        .into_script();
    let tx = bitcoin::Transaction {
        version: 1,
        lock_time: 0,
        input: vec![bitcoin::TxIn {
            previous_output: bitcoin::OutPoint::new(args.txid, args.vout),
            script_sig: Default::default(),
            sequence: bitcoin::blockdata::constants::MAX_SEQUENCE,
            witness: vec![],
        }],
        output: vec![bitcoin::TxOut {
            script_pubkey: out_script,
            value: args.amount,
        }],
    };
    println!(
        "{}",
        hex::encode(bitcoin::consensus::encode::serialize(&tx))
    );
}
