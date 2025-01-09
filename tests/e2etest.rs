#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_inference() {
        let agent = ZeroMLAgent::new(
            "test_agent".to_string(),
            vec![],
            vec![],
        );

        let input = vec![1, 2, 3, 4];
        let inference = agent.generate_inference(input).await.unwrap();
        
        // Generate proof
        let proof = agent.prover.generate_proof(&inference).unwrap();
        
        // Verify proof
        let verification_result = agent.verify_inference(&inference, &proof).await.unwrap();
        assert!(verification_result);
    }

   