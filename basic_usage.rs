use zeroml::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize agent
    let agent = ZeroMLAgent::new(
        "example_agent".to_string(),
        vec![], // Proving key
        vec![], // Verification key
    );

    // Initialize chain connector
    let connector = EthereumConnector::new(
        "0xcontract".to_string(),
        "http://localhost:8545".to_string(),
    );

    // Generate inference
    let input = vec![1, 2, 3, 4];
    let inference = agent.generate_inference(input).await?;
    
    // Generate proof
    let proof = agent.prover.generate_proof(&inference)?;
    
    // Verify locally
    let verification_result = agent.verify_inference(&inference, &proof).await?;
    println!("Local verification result: {}", verification_result);
    
    // Submit to chain
    let tx_hash = connector.submit_inference(&inference, &proof).await?;
    println!("Transaction hash: {}", tx_hash);
    
    // Verify on chain
    let onchain_result = connector.verify_onchain(&tx_hash).await?;
    println!("Onchain verification result: {}", onchain_result);

    Ok(())
}