1. Follow the Lighthouse [steps](https://webump.xyz/lighthouse-guide/SetupLighthouse)

2. Create a merkle root from a whitelist with only the swap contract address
```bash
lighthouse generate-merkle-root ./whitelist.json
```

3. Deploy the new NFTs contract
```bash
lighthouse deploy
```

4. Connect to the swap contract using the [SEI explorer](https://www.seiscan.app/pacific-1) and try to swap the old NFT or the new one