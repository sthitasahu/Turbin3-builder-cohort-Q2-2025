mod programs;
use crate::programs::turbin_prereq::{TurbinProgram,CompleteArgs,UpdateArgs};
 // programs::Turbin3_prereq::{CompleteArgs, TurbinPrereqProgram, UpdateArgs};

#[cfg(test)]
mod tests {

  use std::str::FromStr;
    use solana_sdk::{
        message::Message,
        pubkey::Pubkey,
        signature::{ read_keypair_file, Keypair, Signer },
        system_instruction::transfer,
        system_program,
        transaction::Transaction,
    
      
    };

    use solana_client::rpc_client::RpcClient;
    use crate::programs::turbin_prereq::{TurbinProgram,CompleteArgs,UpdateArgs};
    
    
   
    const RPC_URL: &str = "https://api.devnet.solana.com";

    //  1: Generate a Keypair
    #[test]
    fn keygen() {
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    //airdrop 
     #[test]
    fn airdrop(){
        let keypair=read_keypair_file("dev-wallet.json").expect("Could not find wallet file");
        let client=RpcClient::new(RPC_URL);

        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) { 
            Ok(s) => { 
            println!("Success! Check out your TX here:"); 
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", s.to_string());
            }, 
            Err(e) => println!("Oops, something went wrong: {}", e.to_string()) }; 
            
    }

    #[test]
    fn transfer_sol() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("ARkS9QfNqZf1hfDZsfz7pYTvLDyuNym5dBpYaNq8ZnDx").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);

        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 1_000_000)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash
        );
        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }


    #[test]
    fn empty_wallet() {
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = Pubkey::from_str("ARkS9QfNqZf1hfDZsfz7pYTvLDyuNym5dBpYaNq8ZnDx").unwrap();
        let rpc_client = RpcClient::new(RPC_URL);
        //get the balance of this pubkey
        let balance = rpc_client.get_balance(&keypair.pubkey()).expect("Failed to get balance");
        let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get blockhash");

        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash
        );

        let fee = rpc_client.get_fee_for_message(&message).expect("Failed to get fee");
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &[&keypair],
            recent_blockhash
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction:");
        println!("Success! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
    }

   #[test]
   fn enroll(){
    let rpc_client = RpcClient::new(RPC_URL);
    let signer = read_keypair_file("turbin3-wallet.json").expect("Couldn't find wallet file");
    

    let prereq = TurbinProgram::derive_program_address(
        &[b"prereq", signer.pubkey().to_bytes().as_ref()]
    );   


    let args=CompleteArgs{
        github: b"sthitasahu".to_vec(),
    };


    let blockhash=rpc_client.get_latest_blockhash().expect("Failed to get latest blockhash");

    let transaction = TurbinProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash
    );

    let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Transaction failed");
        println!("Enrollment complete! Check out your TX here: https://explorer.solana.com/tx/{}?cluster=devnet", signature);
  }

}
       

   
    

   
    
