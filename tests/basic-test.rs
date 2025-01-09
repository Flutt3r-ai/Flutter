#[tokio::test]
async fn test_chain_submission() {
    let connector = EthereumConnector::new(
        "0xcontract".to_string(),
        "http://localhost:8545".to_string(),
    );

    let agent = ZeroMLAgent::new(
        "test_agent".to_string(),
        vec![],
        vec![],
    );

    let input = vec![1, 2, 3, 4];
    let inference = agent.generate_inference(input).await.unwrap();
    let proof = agent.prover.generate_proof(&inference).unwrap();

    let tx_hash = connector.submit_inference(&inference, &proof).await.unwrap();
    let verification_result = connector.verify_onchain(&tx_hash).await.unwrap();
    assert!(verification_result);
}
}