# Swap NFTs CosmWASM template

_Original template: [here](https://github.com/CosmWasm/cw-template_

The other day a friend uploaded an NFT collection using [Lighthouse](https://github.com/We-Bump/Lighthouse-cli) to [SEI](https://docs.sei.io/learn/about-sei) (a CosmWASM blockchain) with the wrong metadata uri. He asked me if I could help him fix it. I thought it would be a good idea to create a template for this kind of situation.

## Workflow

NFTs (ERC721 and cw721) are inmutable so there is no way to change the metadata uri from a deployed contract (or they should not be a way. If there is, do not trust the contract). The only way to fix this is to create a new contract with the correct metadata uri and swap the NFTs from the old contract to the new one. The workflow is as follows:

1. User with incorrect-metadata NFT burns the NFT from the old contract.
2. User sends the NFT data to the swap contract.
3. Swap contract checks if the NFT was burned correctly.
4. Swap contract mints a new NFT with the correct metadata uri and sends it to the user.

This way we avoid creating two markets and we keep the uniqueness of the NFTs (if there are 1000 NFTs in supply in the old contract, there will be 1000 NFTs in supply in the new contract if all users swap their NFT). There could be cases where the users do not want to swap their NFTs, in that case, they will keep the incorrect metadata but they will still be able to trade it in marketplaces. A new buyer might want to update the metadata so he will go the swap contract.

## Personal message

When I created this template my friend and I were extremely stressed. If you are in this situation remember something: `It will all pass. You will fix it and you will learn from it.`

I hope this template helps you to fix your NFTs. If you have any questions, feel free to ask me. I will be happy to help you. Disclaimer: I am an EVM developer so I might not be able to help you with CosmWASM specifics but I will do my best to help you.

EVM rocks!!
