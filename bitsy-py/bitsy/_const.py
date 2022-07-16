import web3 as web3_

SQL_NULL = "null"

BITSY_ETH_INFURA_URL = (
    "https://mainnet.infura.io/v3/d43cbbb5c7074d3ea28685326166b2e7"
)

web3 = web3_.Web3(web3_.HTTPProvider(BITSY_ETH_INFURA_URL))
web3.eth.account.enable_unaudited_hdwallet_features()
assert web3.isConnected()
