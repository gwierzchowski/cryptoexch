// use protobuf_codegen_pure::Customize;

fn main() {
    println!("cargo:rerun-if-changed=src/bitbay/trading_ticker/pb/trading_ticker.proto");
    protobuf_codegen_pure::Codegen::new()
        // .customize(Customize {
        //     ..Default::default()
        // })
        .input("src/bitbay/trading_ticker/pb/trading_ticker.proto")
        .include("src/bitbay/trading_ticker/pb")
        .out_dir("src/bitbay/trading_ticker/pb")
        .run()
        .expect("Codegen failed.");

    println!("cargo:rerun-if-changed=src/bitbay/trading_stats/pb/trading_stats.proto");
    protobuf_codegen_pure::Codegen::new()
        .input("src/bitbay/trading_stats/pb/trading_stats.proto")
        .include("src/bitbay/trading_stats/pb")
        .out_dir("src/bitbay/trading_stats/pb")
        .run()
        .expect("Codegen failed.");
}
