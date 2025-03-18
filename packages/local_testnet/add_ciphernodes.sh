#!/bin/bash

CIPHERNODE_ADDRESS_1="0x2546BcD3c84621e976D8185a91A922aE77ECEc30"
CIPHERNODE_ADDRESS_2="0xbDA5747bFD65F08deb54cb465eB87D40e51B197E"
CIPHERNODE_ADDRESS_3="0xdD2FD4581271e230360230F9337D5c0430Bf44C0"

# Add ciphernodes
yarn ciphernode:add --ciphernode-address $CIPHERNODE_ADDRESS_1 --network localhost
yarn ciphernode:add --ciphernode-address $CIPHERNODE_ADDRESS_2 --network localhost
yarn ciphernode:add --ciphernode-address $CIPHERNODE_ADDRESS_3 --network localhost