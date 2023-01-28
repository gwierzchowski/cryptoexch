// use protobuf_codegen_pure::Customize;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    #[cfg(feature = "out_pb")] {
        #[cfg(feature = "mod_zonda")] {
            println!("cargo:rerun-if-changed=src/zonda/trading_ticker/pb/trading_ticker.proto");
            protobuf_codegen_pure::Codegen::new()
                // .customize(Customize {
                //     ..Default::default()
                // })
                .input("src/zonda/trading_ticker/pb/trading_ticker.proto")
                .include("src/zonda/trading_ticker/pb")
                .out_dir("src/zonda/trading_ticker/pb")
                .run()
                .expect("Codegen failed.");

            println!("cargo:rerun-if-changed=src/zonda/trading_stats/pb/trading_stats.proto");
            protobuf_codegen_pure::Codegen::new()
                .input("src/zonda/trading_stats/pb/trading_stats.proto")
                .include("src/zonda/trading_stats/pb")
                .out_dir("src/zonda/trading_stats/pb")
                .run()
                .expect("Codegen failed.");

            println!("cargo:rerun-if-changed=src/zonda/trading_orderbook/pb/trading_orderbook.proto");
            protobuf_codegen_pure::Codegen::new()
                .input("src/zonda/trading_orderbook/pb/trading_orderbook.proto")
                .include("src/zonda/trading_orderbook/pb")
                .out_dir("src/zonda/trading_orderbook/pb")
                .run()
                .expect("Codegen failed.");

            println!("cargo:rerun-if-changed=src/zonda/trading_transactions/pb/trading_transactions.proto");
            protobuf_codegen_pure::Codegen::new()
                .input("src/zonda/trading_transactions/pb/trading_transactions.proto")
                .include("src/zonda/trading_transactions/pb")
                .out_dir("src/zonda/trading_transactions/pb")
                .run()
                .expect("Codegen failed.");

            println!("cargo:rerun-if-changed=src/zonda/trading_candle/pb/trading_candle.proto");
            protobuf_codegen_pure::Codegen::new()
                .input("src/zonda/trading_candle/pb/trading_candle.proto")
                .include("src/zonda/trading_candle/pb")
                .out_dir("src/zonda/trading_candle/pb")
                .run()
                .expect("Codegen failed.");
        }
    }
}
